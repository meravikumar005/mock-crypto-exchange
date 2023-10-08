<script lang="ts">
  import { Svroller } from "svrollbar";
  import { type IStockList } from "./stock.interface";

  let stockList: IStockList[] = [];
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
    stockList = [...stockList, ...data];
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
</script>

<main class="stock-list">
  <div class="search-box">
    <input class="search-input" type="text" placeholder="Search" />
  </div>
  <Svroller width="100%" height="95%">
    {#each stockList.slice(0, 100) as stock}
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
      </div>
    {/each}
  </Svroller>
</main>

<style>
  .stock-list {
    height: 100%;
  }

  .search-box {
    height: 35px;
    padding: 10px;
  }

  .search-input {
    width: 100%;
    height: 100%;
    border-radius: 6px;
    border: none;
    padding-left: 5px;
    font-size: 14px;
    background-color: #1e2433;
    color: #fff;
  }

  .search-input:focus {
    outline: none;
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

  .selected-stock {
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
    max-width: 80px;
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
    float: inline-end;
  }

  .change-rate-down {
    color: #f6685e !important;
  }

  .change-rate-up {
    color: #66c37b;
  }
</style>
