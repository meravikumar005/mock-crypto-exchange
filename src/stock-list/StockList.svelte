<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Svroller } from "svrollbar";
  import { type IStockList } from "./stock.interface";
  import { TokenStore, TradeClose } from "../store/store";
  import Trade from "../trade/Trade.svelte";
  let showTrade = false;
  let stockList: IStockList[] = [];
  let selectedToken: IStockList;
  let tradeType = "BUY";
  const ws = new WebSocket("wss://stream.wazirx.com/stream");

  ws.addEventListener("open", () => {
    ws.send(JSON.stringify({ event: "subscribe", streams: ["!ticker@arr"] }));
  });

  setInterval(() => {
    ws.send(JSON.stringify({ event: "ping" }));
  }, 1500 * 1000);

  ws.onmessage = (e) => {
    const res = JSON.parse(e.data);
    const data: IStockList[] = res.data;
    if (Array.isArray(data)) {
      stockList = [...stockList, ...data];
    }
  };

  const getPriceDiffPer = (stock: IStockList) => {
    const change = parseFloat(stock.b) - parseFloat(stock.o);
    const changePer = (change / parseFloat(stock.b)) * 100;
    return changePer.toFixed(2);
  };

  const priceChange = (stock: IStockList) => {
    const change = parseFloat(stock.b) - parseFloat(stock.o);
    if (change < 0) {
      return "UP";
    } else {
      return "Down";
    }
  };

  const selectToken = (token: IStockList, type: string) => {
    showTrade = !showTrade;
    selectedToken = token;
    tradeType = type;
  };

  TradeClose.subscribe((d) => {
    showTrade = d;
    TradeClose.set(true);
  });
</script>

<main class="stock-list">
  <div class="list-header">
    <div class="h-stock-name">Token</div>
    <div class="h-stock-rate">Buy</div>
    <div class="h-stock-rate">Sell</div>
    <div class="h-stock-rate">High</div>
    <div class="h-stock-rate">Low</div>
    <div class="h-stock-rate">Open</div>
    <div class="h-stock-rate">Action</div>
  </div>
  <Svroller width="100%" height="90%">
    {#each stockList.slice(0, 300) as stock}
      <div class="stock-item">
        <div class="stock-logo">
          <img src="https://media.wazirx.com/media/{stock.u}/84.png" alt="" />
        </div>
        <div class="stock-name">{stock.u.toUpperCase()}</div>
        <div class="stock-price">
          <p class="current-rate">{stock.b}</p>
          <p
            class="change-rate {priceChange(stock) === 'UP'
              ? 'change-rate-up'
              : 'change-rate-down'}"
          >
            <span>{priceChange(stock) === "UP" ? "▲" : "▼"}</span>
            {getPriceDiffPer(stock)}
          </p>
        </div>
        <div class="stock-price">
          <p class="current-rate">{stock.a}</p>
        </div>
        <div class="stock-price">
          <p class="current-rate">{stock.h}</p>
        </div>
        <div class="stock-price">
          <p class="current-rate">{stock.l}</p>
        </div>
        <div class="stock-price">
          <p class="current-rate">{stock.o}</p>
        </div>
        <div class="stock-price">
          <button class="btn-buy" on:click={() => selectToken(stock, "BUY")}
            >Buy</button
          >
          <button class="btn-sell" on:click={() => selectToken(stock, "SELL")}
            >Sell</button
          >
        </div>
      </div>
    {/each}
  </Svroller>

  {#if !showTrade}
    <div class="trade-view">
      <Trade token={selectedToken} {tradeType} />
    </div>
  {/if}
</main>

<style>
  .stock-list {
    height: 100%;
  }
  .list-header {
    display: flex;
    background-color: #1e2433;
    align-items: center;
    height: 50px;
  }

  .h-stock-name {
    flex: 1;
    max-width: 840px;
    margin-left: 10px;
    font-size: 15px;
    font-weight: 700;
  }

  .h-stock-rate {
    width: 90px;
    font-size: 15px;
    font-weight: 700;
  }

  .stock-item {
    display: flex;
    gap: 10px;
    justify-content: center;
    align-items: center;
    border-top: 1px solid rgba(102, 102, 102, 0.68);
    padding: 10px;
    cursor: pointer;
  }

  .stock-item:last-child {
    border-bottom: 1px solid rgba(102, 102, 102, 0.68);
  }

  .stock-item:hover {
    background-color: #1a2c61;
  }

  .stock-logo {
    height: 16px;
    width: 16px;
  }

  .stock-logo img {
    height: 100%;
    width: 100%;
    filter: saturate(0) brightness(1.75);
  }

  .stock-name {
    flex: 1;
    font-size: 12px;
    font-weight: 700;
  }

  .stock-price {
    width: 90px;
    font-weight: 700;
  }

  .current-rate {
    margin: 0;
    font-size: 12px;
  }

  .change-rate {
    margin: 0;
    font-size: 10px;
    color: #999ca3;
  }

  .change-rate-down {
    color: #f6685e !important;
  }

  .change-rate-up {
    color: #66c37b;
  }

  .btn-buy {
    height: 24px;
    background-color: rgb(240, 255, 240);
    border-radius: 5px;
    outline: none;
    border: none;
    background-color: green;
    color: white;
    cursor: pointer;
  }

  .btn-sell {
    height: 24px;
    border-radius: 5px;
    outline: none;
    border: none;
    background-color: rgb(223, 17, 17);
    color: white;
    cursor: pointer;
  }

  .trade-view {
    position: absolute;
    position: fixed;
    right: 0;
    top: 105px;
    height: calc(100vh - 105px);
  }
</style>
