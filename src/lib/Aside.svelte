<script>
  import { TextBlock, TextBox, ContentDialog, Button, ListItem, InfoBadge } from "fluent-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  let open = false;
  let file_name;
  let del_file_modal = false;
  let del_file_name;

  export let listFiles;
  export let selectFile;
  function select_file(i) {
    selectFile = i;
  }

  // Creates a new file for the user with the specified file name and reloads the page.
  async function create_file() {
    open = false
    listFiles = JSON.parse(await invoke('create_file_user', {fileName: file_name}));
    window.location.reload();
	}

  // Prompts the user for confirmation to delete the file with the given file name.
  async function delete_file_quest(f) {
    del_file_modal = true;
    del_file_name = f;
	}

  // Deletes the file with given file name and updates the list of files available to the user.
  async function delete_file(del_file_name) {
    del_file_modal = false;
    file_name = del_file_name;
    listFiles = JSON.parse(await invoke('delete_file_user', {fileName: file_name}));
	}
</script>



<div class="aside">
  <ContentDialog bind:open>
    <div class="flex" style="display: flex; flex-direction: row;">
      <TextBox type="text" placeholder="Write the file name." bind:value={file_name} />
      <Button on:click={() => create_file(file_name)} style="cursor: pointer; width: 130px; margin-left: 1em" variant="accent">Create</Button>
      <Button on:click={() => (open = false)} style="cursor: pointer; width: 120px; margin-left: 1em">Close</Button>
    </div>
  </ContentDialog>

  <ContentDialog bind:open={del_file_modal}>
    <div class="flex" style="display: flex; flex-direction: row;">
      <div style="margin-right: auto"><TextBlock variant="subtitle">Delete a file <span style="color:#62cdfe;">{del_file_name}</span>?</TextBlock></div>
      <div><Button style="cursor: pointer" on:click={() => delete_file(del_file_name)} variant="hyperlink">Delete</Button></div>
      <div><Button style="cursor: pointer" on:click={() => (del_file_modal=false)} variant="hyperlink">No</Button></div>
    </div>
  </ContentDialog>


  <div style="display:flex; justify-content: space-between; padding: 0.5em 1em">
    <div style="margin-right: auto"><TextBlock variant="subtitle">Files</TextBlock></div>
    <div><Button style="cursor: pointer" on:click={() => (open = true)} variant="hyperlink">Create file</Button></div>
  </div>

  {#if listFiles && typeof listFiles !== 'string'}
      {#each listFiles as f, i}

        <ListItem style="cursor: pointer; position: relative; z-index: 9;"  selected={selectFile === i ? true : false} on:click={() => select_file(i)}>
          <span>{f.file_name}</span>
          <InfoBadge class="del_file" severity="information" on:click={() => delete_file_quest(f.file_name)}>
            <svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" viewBox="172 171 683 683" class="svelte-106nxdf"><path fill="currentColor" d="M512.5,587.5L262.5,838C252.167,848.333 239.5,853.5 224.5,853.5C209.5,853.5 196.917,848.417 186.75,838.25C176.583,828.083 171.5,815.5 171.5,800.5C171.5,785.5 176.667,772.833 187,762.5L437,512L187,262C176.667,251.667 171.5,239.167 171.5,224.5C171.5,217.167 172.833,210.167 175.5,203.5C178.167,196.833 181.917,191.167 186.75,186.5C191.583,181.833 197.167,178.083 203.5,175.25C209.833,172.417 216.833,171 224.5,171C239.167,171 251.667,176.167 262,186.5L512.5,437L762.5,186.5C773.167,175.833 785.833,170.5 800.5,170.5C807.833,170.5 814.75,171.917 821.25,174.75C827.75,177.583 833.417,181.417 838.25,186.25C843.083,191.083 846.833,196.75 849.5,203.25C852.167,209.75 853.5,216.667 853.5,224C853.5,238.667 848.333,251.167 838,261.5L587.5,512L838,762.5C848.667,773.167 854,785.833 854,800.5C854,807.833 852.583,814.667 849.75,821C846.917,827.333 843.083,832.917 838.25,837.75C833.417,842.583 827.75,846.417 821.25,849.25C814.75,852.083 807.833,853.5 800.5,853.5C785.5,853.5 772.833,848.333 762.5,838Z"></path></svg>
          </InfoBadge>
        </ListItem>

      {/each}
  {/if}
</div>

<style>
  .aside {
      height: 100vh;
      width: 200px;
      border-right: 1px solid #363746;
      background: #292b3a;
      position: fixed;
      z-index: 10;
      top: 0;
      left: 0;
  }
   .flex {
    display: flex;
    flex-direction: column;
  }
  :global(.del_file) {
      position: absolute;
      top:9px;
      right: 10px;
      z-index: 11;
      opacity: 0.1;
  }
  :global(.del_file):hover {
    opacity: 1;
  }

</style>