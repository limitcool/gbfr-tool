<script lang="ts">
  import FileDrop from "svelte-tauri-filedrop";
  import { invoke } from "@tauri-apps/api/tauri";
  import Swal from "sweetalert2";
  import { addMessages, init, getLocaleFromNavigator, _ } from "svelte-i18n";
  import en from "../locales/en.json";
  import zhCN from "../locales/zh-CN.json";
  addMessages("en", en);
  addMessages("zh-CN", zhCN);
  init({
    fallbackLocale: "en",
    initialLocale: getLocaleFromNavigator(),
  });
  // import { fs } from "@tauri-apps/api";
  console.log(getLocaleFromNavigator());
  let steamId;
  let savePath = "";
  async function save() {
    // if (isNaN(steamId)) {
    //   alert("steam Id 输入错误");
    //   return;
    // }
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("save", { steamId, savePath })
      .then((message: string) => {
        console.log(message);
        // alert(message);
        Swal.fire({
          title: "Success!",
          icon: "success",
          text: message,
        });
      })
      .catch((error) => {
        // alert(error);
        Swal.fire({
          title: "Error!",
          text: error,
          icon: "error",
          confirmButtonText: "continue",
        });
      });
  }

  async function open(paths: string[]) {
    savePath = paths[0];
    console.log(`Selected path: ${savePath}`);
  }
  function handleClearPath() {
    savePath = "";
  }
</script>

<div>
  <FileDrop extensions={["dat"]} handleFiles={open} let:files>
    <div class="dropzone" class:droppable={files.length > 0}>
      <h2>
        {$_("file_drag_and_drop_instruction")}
      </h2>
    </div>
  </FileDrop>
  <form class="row" on:submit|preventDefault={save}>
    <input
      id="steam-id-input"
      placeholder={$_("steam_id_input_prompt")}
      required
      bind:value={steamId}
    />

    <button type="submit">{$_("modify_archive")}</button>
  </form>
  <button class="clear-button" on:click={handleClearPath}
    >{$_("choose_archive_again")}</button
  >
</div>

<style>
  .dropzone {
    margin: 20px;
    padding: 20px;
    background: #f7f7f7;
    border: 2px dashed #3498db;
    border-radius: 8px;
    text-align: center;
    color: #555;
    cursor: pointer;
  }

  .dropzone:hover {
    background: #f0f0f0;
  }

  .droppable {
    background: #d6dff0;
    border-color: #2980b9;
  }
  .clear-button {
    margin-top: 10px;
    position: left;
    /* width: 50px; */
  }
</style>
