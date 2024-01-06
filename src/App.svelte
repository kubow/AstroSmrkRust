<script>
  import InputGroup from './lib/InputGroup.svelte'
  import Navigation from './lib/Navigation.svelte'
  import Zodiac from './lib/Zodiac.svelte';
  let innerWidth = 0
	let innerHeight = 0
	
	//$: condition = innerWidth*1.33 <= innerHeight
</script>

<svelte:window bind:innerWidth bind:innerHeight />

<main class="container">
  <Navigation></Navigation>
  <div class="content">
    <div class="setting">
      <InputGroup></InputGroup>
    </div>
    <Zodiac></Zodiac>
  </div>
</main>

<style>
  .container{
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    height: 100%;
  }
  
  .setting{
    width: 100%;
    order: -1;
    flex: 0 0 20vw;
  }
  .content{
    min-height: calc(100vh - 40vh);
    background-color: #f0f0f0;
  }
</style>


