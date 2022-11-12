# KittenVM
大抵是一个想象力丰富的LVM吧。

## 使用
### Rust
'''
KittenVM = "1.0.0-1"
'''
### C/C++
确保KittenVM.lib要和libKittenVM.d在同一目录下；如果使用CMake之类的构建工具，确保正确引用！
[Staticlib下载](https://gitee.com/Shirasawa-CN/kitten-vm/releases)

## 特性
本玩具具有许多奇奇怪怪的脑洞特性，请移步到[README](./docs/README.md)

# 使用
## 创建空间
```
new name
```
创建一个名为name的内存空间

## 移动数据
```
mov name,4
```
将数字4移动到name空间

```
mov name1,name2
```
将name2的值移动到name1，此时name2变成None

## 内存回收
```
add_gc a
```
将a添加到回收列表中
```
free
```
清理回收列表中的内存

## 运算
add and div mul or sll sra sud xor 这些运算指令的格式均如下

```
expr rs1,rs2,target
```