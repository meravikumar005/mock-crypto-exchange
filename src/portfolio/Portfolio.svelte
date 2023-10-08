<script lang="ts">
  import { TokenStore } from "../store/store";
</script>

<main>
  <div class="token-info">
    <span class="portfolio-title">Portfolio</span>
  </div>
  <div class="list-header">
    <div class="h-stock-name">Token</div>
    <div class="h-stock-rate">Qty</div>
    <div class="h-stock-rate">Rate</div>
    <div class="h-stock-rate">Type</div>
  </div>
  <div class="stock-item">
    <div class="stock-logo">
      <img src="https://media.wazirx.com/media/btcinr/84.png" alt="" />
    </div>
    <div class="stock-name">BTC</div>
    <div class="stock-price">
      <p class="current-rate">340</p>
    </div>
    <div class="stock-price">
      <p class="current-rate">125.89</p>
    </div>

    <div class="stock-price">
      <p class="current-rate token-buy">Buy</p>
    </div>
  </div>

  <div class="stock-item">
    <div class="stock-logo">
      <img src="https://media.wazirx.com/media/btcinr/84.png" alt="" />
    </div>
    <div class="stock-name">BTC</div>
    <div class="stock-price">
      <p class="current-rate">200</p>
    </div>
    <div class="stock-price">
      <p class="current-rate">125.89</p>
    </div>

    <div class="stock-price">
      <p class="current-rate token-sell">Sell</p>
    </div>
  </div>
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
</style>
