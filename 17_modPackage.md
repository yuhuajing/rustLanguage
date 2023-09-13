# Mod package

rust的分包管理 mod，公共包则为 pub mod

每个文件都是一个包。同目录下直接使用文件名作为包名。
![](picture/moddiffpack.png)
不同目录下，每个目录下都需要 mod.rs 作为包入口，显示说明当前目录下的包名以及包的类型。
![](picture/modpack.png)
