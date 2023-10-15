<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { TradeClose } from "../store/store";
  import { type IStockList } from "../stock-list/stock.interface";
  import { EventService } from "../event/event";
  export let token:IStockList;
  export let tradeType = "BUY";
  let tradeQty:string = "0";
  let tradePrice = parseFloat(token.a);
  let tokenName = String(token.s).toUpperCase();

  const eventService = new EventService();

  const closeTrade = () => {
    TradeClose.set(false);
  };

  const makeTrade = () => {
    const trade = {
      token: token.u,
      qty: parseInt(tradeQty),
      rate: tradePrice,
      trade_type: tradeType,
    }

    invoke("create_trade", { trade }).then(res => {
      eventService.dispatchEvent({type:"TRADE_COMPLETED"})
      closeTrade();
    })
  }
</script>

<main class="trade-wrapper">
  <div class="trade-header">
    <p class="btn-sec">
      <button class="trade-close" on:click={closeTrade}>x</button>
    </p>
    <span class="trade-title">Place order</span>
  </div>
  <div class="trade-form">
    <p class="token-name">{tokenName}</p>
    <div class="input-box">
      <input class="text-input" type="text" placeholder="Qty" bind:value={tradeQty} />
    </div>

    <div class="input-box">
      <input class="text-input" type="text" disabled placeholder="Rate" bind:value={tradePrice} />
    </div>

    <div class="trade-btn">
      {#if tradeType === "BUY"}
        <button class="btn-buy" on:click={() => makeTrade()}>Buy</button>
      {/if}

      {#if tradeType === "SELL"}
        <button class="btn-sell" on:click={() => makeTrade()}>Sell</button>
      {/if}
    </div>
  </div>
  <div class="trade-value">
    <span>
      {#if parseInt(tradeQty) > 0}
        Total INR:- {parseInt(tradeQty) * tradePrice}
      {/if}
    </span>
  </div>
</main>

<style>
  .trade-wrapper {
    background-color: #1e2433;
    width: 250px;
    height: 100%;
    box-shadow: 2px 0 7px -1px #171010;
  }

  .trade-header {
    background-color: #2d3446;
    padding: 10px;
    text-align: center;
  }

  .btn-sec {
    margin: 0;
  }

  .trade-close {
    background-color: transparent;
    border: none;
    outline: none;
    color: #fff;
    font-size: 14px;
    float: right;
    cursor: pointer;
  }

  .trade-title {
    font-size: 12px;
    font-weight: 700;
  }

  .input-box {
    height: 30px;
    padding: 10px;
  }

  .text-input {
    width: 100%;
    height: 100%;
    border-radius: 6px;
    border: none;
    padding-left: 5px;
    font-size: 14px;
    background-color: #2d3446;
    color: #fff;
  }

  .token-name {
    font-size: 14px;
    font-weight: 700;
    text-align: center;
  }

  .trade-btn {
    padding-left: 10px;
  }

  .trade-btn button {
    width: 100%;
    height: 32px;
    border-radius: 8px;
    border: none;
    outline: none;
    cursor: pointer;
    margin-right: 5px;
    color: #fff;
  }

  .btn-sell {
    background-color: #f6685e !important;
  }

  .btn-buy {
    background-color: #66c37b;
  }

  .trade-value {
    font-size: 14px;
    font-weight: 700;
    background-color: #2d3446;
    padding: 10px;
    text-align: center;
    position: absolute;
    bottom: 0;
    width: 100%;
  }
</style>
