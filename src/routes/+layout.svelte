<script lang="ts">
  import '../app.css';

  let { children } = $props();
</script>

<div class="app-container">
  {@render children()}
</div>

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }
</style>
