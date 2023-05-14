using System.Diagnostics;
using Wasmtime;

using var engine = new Engine();
using var store = new Store(engine);
using var module = Module.FromFile(engine, "/home/rainer/github/rust-samples/rust-csharp/rust-wasm-module/pkg/rust_wasm_module_bg.wasm");
using var linker = new Linker(engine);
var instance = linker.Instantiate(store, module);

var alloc = instance.GetFunction<int, int>("__wbindgen_malloc");
Debug.Assert(alloc != null);
var free = instance.GetAction<int, int>("__wbindgen_free");
Debug.Assert(free != null);

var add = instance.GetFunction<int, int, int>("add")!;
Debug.Assert(add != null);
var result = add(1, 2);
Console.WriteLine(result);

var addr2 = alloc(12);
free(addr2, 12);

ReadOnlySpan<byte> hello = "Hello World!\0"u8;
System.Console.WriteLine(hello.Length);
var addr = alloc(hello.Length);
System.Console.WriteLine(addr);
var memory = instance.GetMemory("memory")!;
Debug.Assert(memory != null);
for (var i = 0; i < hello.Length; i++)
{
    memory.WriteByte(addr + i, hello[i]);
}

var strlen = instance.GetFunction<int, int, int>("get_len_of_str")!;
Debug.Assert(strlen != null);
var len = strlen(addr, 1);
Console.WriteLine(len);
//free(addr, hello.Length);
