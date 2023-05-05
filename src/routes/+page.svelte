
<script>
  import CodeMirror from "svelte-codemirror-editor";
  import { javascript } from "@codemirror/lang-javascript";
  import { oneDark } from "@codemirror/theme-one-dark";
	import Aside from "$lib/Aside.svelte";
	import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri'
	import { TextBox, TextBoxButton, ContentDialog } from "fluent-svelte";

	let open = true;
	let value;
  let data;
	let i_file = "";

	// Function to retrieve password data from Tauri backend
  async function get_pass() {data = await invoke('get_pass')}
  onMount(() => {get_pass();});

	// Function to create new password data in Tauri backend
  async function create_password() {
    data = await invoke('create_password', {password: value});
  }

	// Function to check if the entered password is correct by comparing it with stored password data in Tauri backend
  async function chek_password() {
    data = await invoke('chek_password', {password: value});

    if (data != 'Password incorrect.') {
      data = JSON.parse(data);
    }
  }

	// Function to write updated password data to Tauri backend
	async function changeHandler() {
    let write_data = JSON.stringify(data);
		await invoke('write_data', {data: write_data});
	}
</script>
<main>
	<Aside listFiles={data} bind:selectFile={i_file} />

	<div class="container">

		{#if data === 'Password exists.' || data === 'Password incorrect.'}
			<ContentDialog bind:open>
				<TextBox type="password" placeholder="Enter password!" bind:value on:keydown={(event) => {if (event.key === "Enter") {chek_password(value);}}}>
					<TextBoxButton slot="buttons" on:click={() => chek_password(value)}>
						<svg width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
							<path d="M7.85355 0.146447C7.65829 -0.0488155 7.34171 -0.0488155 7.14645 0.146447C6.95118 0.341709 6.95118 0.658291 7.14645 0.853553L8.29603 2.00314C4.80056 2.11088 2 4.97839 2 8.5C2 12.0899 4.91015 15 8.5 15C12.0899 15 15 12.0899 15 8.5C15 8.48656 15 8.47313 14.9999 8.45971C14.9983 8.2001 14.7805 8 14.5209 8H14.4782C14.2093 8 14 8.23107 14 8.5C14 11.5376 11.5376 14 8.5 14C5.46243 14 3 11.5376 3 8.5C3 5.53311 5.34917 3.11491 8.28892 3.00398L7.14645 4.14645C6.95118 4.34171 6.95118 4.65829 7.14645 4.85355C7.34171 5.04882 7.65829 5.04882 7.85355 4.85355L9.85355 2.85355C10.0488 2.65829 10.0488 2.34171 9.85355 2.14645L7.85355 0.146447ZM11.8536 6.14645C12.0488 6.34171 12.0488 6.65829 11.8536 6.85355L8.85355 9.85355C8.65829 10.0488 8.34171 10.0488 8.14645 9.85355L6.64645 8.35355C6.45118 8.15829 6.45118 7.84171 6.64645 7.64645C6.84171 7.45118 7.15829 7.45118 7.35355 7.64645L8.5 8.79289L11.1464 6.14645C11.3417 5.95118 11.6583 5.95118 11.8536 6.14645Z" fill="currentColor" />
						</svg>
					</TextBoxButton>
				</TextBox>

				{#if data === 'Password incorrect.'}
					<p>Password incorrect.</p>
				{:else if data === 'Password does not exist.'}
					<p>Password does not exist.</p>
				{/if}
			</ContentDialog>
		{:else if data === 'Password does not exist.'}
			<ContentDialog bind:open>
				<TextBox type="password" placeholder="Create password!" bind:value  on:keydown={(event) => {if (event.key === "Enter") {create_password(value);}}}>
					<TextBoxButton slot="buttons" on:click={() => create_password(value)}>
						<svg width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
							<path d="M7.85355 0.146447C7.65829 -0.0488155 7.34171 -0.0488155 7.14645 0.146447C6.95118 0.341709 6.95118 0.658291 7.14645 0.853553L8.29603 2.00314C4.80056 2.11088 2 4.97839 2 8.5C2 12.0899 4.91015 15 8.5 15C12.0899 15 15 12.0899 15 8.5C15 8.48656 15 8.47313 14.9999 8.45971C14.9983 8.2001 14.7805 8 14.5209 8H14.4782C14.2093 8 14 8.23107 14 8.5C14 11.5376 11.5376 14 8.5 14C5.46243 14 3 11.5376 3 8.5C3 5.53311 5.34917 3.11491 8.28892 3.00398L7.14645 4.14645C6.95118 4.34171 6.95118 4.65829 7.14645 4.85355C7.34171 5.04882 7.65829 5.04882 7.85355 4.85355L9.85355 2.85355C10.0488 2.65829 10.0488 2.34171 9.85355 2.14645L7.85355 0.146447ZM11.8536 6.14645C12.0488 6.34171 12.0488 6.65829 11.8536 6.85355L8.85355 9.85355C8.65829 10.0488 8.34171 10.0488 8.14645 9.85355L6.64645 8.35355C6.45118 8.15829 6.45118 7.84171 6.64645 7.64645C6.84171 7.45118 7.15829 7.45118 7.35355 7.64645L8.5 8.79289L11.1464 6.14645C11.3417 5.95118 11.6583 5.95118 11.8536 6.14645Z" fill="currentColor" />
						</svg>
					</TextBoxButton>
				</TextBox>

				{#if data === 'Password incorrect.'}
					<p>Password incorrect.</p>
				{:else if data === 'Password does not exist.'}
					<p>Password does not exist.</p>
				{/if}
			</ContentDialog>
		{/if}

		{#if typeof i_file !== 'string'}
			<div class="code">
			<CodeMirror
					bind:value={data[i_file].data}
					doc={data[i_file].data}
					lang={javascript()}
					theme={oneDark}
					on:change={changeHandler}
				/>
			</div>
		{/if}
	</div>
</main>

<style>
	main {
		display: flex;
		height: 100%;
		color: #eee;
		flex: 1;
		flex-direction: column;
		width: 100%;
		margin: 0 auto;
		box-sizing: border-box;
	}
	.container {flex: 1;padding: 1em;font-size: 1.3em;margin-left: 200px;}
	.code {border: 1px solid #363746;}
</style>