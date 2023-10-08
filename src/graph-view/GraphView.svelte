<script lang="ts">
  import { onMount } from "svelte";
  import * as Highcharts from "highcharts";
  import stock from "highcharts/modules/stock";
  import { data } from "./data";
  stock(Highcharts);

  onMount(() => {
    const container: any = document.getElementById("container");
    Highcharts.stockChart(container, {
      chart: {
        height: "40%",
        backgroundColor: "#2e3340",
        style: {
          color: "white",
        },
      },
      rangeSelector: {
        selected: 1,
      },
      navigator: {
        enabled: false,
      },
      scrollbar: {
        enabled: false,
      },
      title: {
        text: "AAPL Stock Price",
        style:{
          color:"#fff"
        }
      },
      xAxis: {
        labels: {
          style: {
            color: "white", // Set x-axis text color
          },
        },
      },
      yAxis: {
        labels: {
          enabled: false,
          style: {
            color: "white", // Set x-axis text color
          },
        },
      },
      plotOptions: {
        candlestick: {
          color: "red", // Set the color for falling candles
          upColor: "green", // Set the color for rising candles
        },
      },
      series: [
        {
          name: "AAPL",
          type: "candlestick",
          data: data,
          tooltip: {
            valueDecimals: 2,
          },
        },
      ],
    });
  });
</script>

<main class="graph-container">
  <div id="container" />
</main>

<style>
  .graph-container {
    height: 100%;
    background-color: #2e3340;
  }
</style>
