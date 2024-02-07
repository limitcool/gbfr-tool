<script lang="ts">
  import FileDrop from "svelte-tauri-filedrop";
  import { invoke } from "@tauri-apps/api/tauri";
  import Swal from "sweetalert2";
  // import { fs } from "@tauri-apps/api";

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
        拖拽要修改的存档文件至此处。这份存档可能来自他人分享或是您之前更高等级的进度。
      </h2>
    </div>
  </FileDrop>
  <form class="row" on:submit|preventDefault={save}>
    <input
      id="steam-id-input"
      placeholder="输入你的Steam id..."
      required
      bind:value={steamId}
    />

    <button type="submit">修改</button>
  </form>
  <button class="clear-button" on:click={handleClearPath}>重新选择存档</button>
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
