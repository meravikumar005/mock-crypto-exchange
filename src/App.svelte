<script lang="ts">
  import StockListSidebar from "./stock-list/StockListSidebar.svelte";
  import GraphView from "./graph-view/GraphView.svelte";
  import Portfolio from "./portfolio/Portfolio.svelte";
</script>

<main class="app-wrapper">
  <div class="stock-list-sidebar">
    <StockListSidebar />
  </div>

  <div class="main-container">
    <div class="graph-view">
      <GraphView />
    </div>
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
    gap: 10px;
  }

  .stock-list-sidebar {
    width: 250px;
    height: 100%;
    min-width: 250px;
    background-color: #2d3446;
  }

  .main-container {
    flex: 1 auto;
    background-color: #2d3446;
  }

  .graph-view {
    height: 60%;
  }
</style>
