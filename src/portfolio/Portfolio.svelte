<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { EventService } from "../event/event";
  import { type IPortfolio } from "./portfolio";
  const eventService = new EventService();
  let tokenList:IPortfolio[] = [];

  invoke("get_trade_list").then(res => {
      const trade = res as IPortfolio[];
      tokenList = trade;
    })

  eventService.customEvent.subscribe(data => {
    invoke("get_trade_list").then(res => {
      const trade = res as IPortfolio[];
      tokenList = [...tokenList, ...trade];
    })
  })

  const deleteTrade = (id:number) => {
    invoke("delete_trade", { id }).then(() => {
      let index = tokenList.findIndex(t => t.id === id);
      tokenList.splice(index, 1);
      tokenList = tokenList;
    })
  }

</script>

<main>
  <div class="token-info">
    <span class="portfolio-title">Portfolio</span>
  </div>

  {#if tokenList.length > 0}
    <div class="list-header">
      <div class="h-stock-name">Token</div>
      <div class="h-stock-rate">Qty</div>
      <div class="h-stock-rate">Rate</div>
      <div class="h-stock-rate">Type</div>
    </div>
  {/if}
{#each tokenList as token}
  <div class="stock-item">
    <div class="stock-logo">
      <img src="https://media.wazirx.com/media/btcinr/84.png" alt="" />
    </div>
    <div class="stock-name">{token.token.toUpperCase()}</div>
    <div class="stock-price">
      <p class="current-rate">{token.qty}</p>
    </div>
    <div class="stock-price">
      <p class="current-rate">{token.rate.toFixed(3)}</p>
    </div>

    <div class="stock-price">
      {#if token.trade_type === "BUY"}
        <p class="current-rate token-buy">{token.trade_type}</p>
      {/if}
      {#if token.trade_type === "SELL"}
      <p class="current-rate token-sell">{token.trade_type}</p>
    {/if}
    </div>
    <button class="btn-sell" on:click={() => deleteTrade(token.id)}>x</button>
  </div>
{/each}
</main>

<style>
  .token-info {
    background-color: #3d455a;
    padding: 5px 0px;
    text-align: center;
  }

  .portfolio-title {
    font-size: 14px;
    font-weight: 700;
  }

  .list-header {
    display: flex;
    background-color: #1e2433;
    align-items: center;
    height: 40px;
  }

  .h-stock-name {
    flex: 1;
    margin-left: 10px;
    font-size: 14px;
    font-weight: 700;
  }

  .h-stock-rate {
    width: 90px;
    font-size: 14px;
    font-weight: 700;
  }

  .stock-item {
    display: flex;
    gap: 10px;
    justify-content: center;
    align-items: center;
    border-top: 1px solid rgba(102, 102, 102, 0.68);
    cursor: pointer;
  }

  .stock-item:last-child {
    border-bottom: 1px solid rgba(102, 102, 102, 0.68);
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
  }

  .token-buy {
    color: green;
  }

  .token-sell {
    color: rgb(223, 17, 17);
  }

  .btn-sell {
    height: 22px;
    border-radius: 5px;
    outline: none;
    border: none;
    background-color: rgb(223, 17, 17);
    color: white;
    cursor: pointer;
    margin-right: 5px;
  }
</style>
