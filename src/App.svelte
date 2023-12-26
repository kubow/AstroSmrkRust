<script>
  import InputGroup from './lib/InputGroup.svelte'
  import Zodiac from './lib/Zodiac.svelte';
</script>

<main class="container">
  <nav class="navigation">
    <ul>
       <li><a href="/">Soubor</a></li>
       <li><a href="/">Změny</a></li>
       <li><a href="/">Ukončit</a></li>
    </ul>
 </nav>
 <div id="content">
  <div class="menu">
    <InputGroup></InputGroup>
   </div>
  <Zodiac></Zodiac>
 </div>
</main>

<style>
  .container{
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;;
    height: 100%;
  }
  .navigation{
    display: flex;
    flex-direction: row;
    background-color: #333;
    justify-content: center;
  } 
  .menu{
    width: 100%;
    order: -1;
    flex: 0 0 20vw;
  }
  .content{
    min-height: calc(100vh - 40vh);
  background-color: #f0f0f0;
  }
</style>


