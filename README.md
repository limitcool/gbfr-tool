# GBFR-Tool

`GBFR-Tool` 是一个用于 `Granblue Fantasy Relink` 存档修改的工具。

![界面图片](https://github.com/limitcool/gbfr-tool/blob/main/assets/screenshot.png?raw=true)

## 使用方法

1. **拖拽存档文件:**
   将需要修改的存档文件拖拽至工具界面。

2. **在 Steam 获取 Steam ID:**
    Steam ID 示例: 76500000000000。
   填入 Steam ID 后点击修改，将在文件目录下生成一个 `new_save.dat` 的文件。

3. **替换存档**

    ```bash
    进入目录 %LOCALAPPDATA%\GBFR\Saved\SaveGames
    将 new_save.dat 重命名为 SaveData1.dat 复制进去
    ```

修改方法源自：[巴哈姆特论坛](https://forum.gamer.com.tw/C.php?bsn=25204&snA=13377)。

## 贡献

如果您发现任何问题或有改进的建议，请随时提出 `issue` 或提交 `pull request`。

## 许可证

[GPL License](LICENSE)
