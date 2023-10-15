<script lang="ts">
  import StockListSidebar from "./stock-list/StockList.svelte";
  import Portfolio from "./portfolio/Portfolio.svelte";
</script>

<main class="app-wrapper">
  <div class="stock-list-sidebar">
    <StockListSidebar />
  </div>

  <div class="main-container">
    <div class="portfolio-position">
      <Portfolio />
    </div>
  </div>
</main>

<style>
  .app-wrapper {
    background-color: #101623;
    height: 100vh;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .stock-list-sidebar {
    height: 70%;
    background-color: #2d3446;
  }

  .main-container {
    flex: 1 auto;
    background-color: #2d3446;
  }

</style>
