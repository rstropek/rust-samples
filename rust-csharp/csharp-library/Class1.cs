using System.Runtime.InteropServices;

namespace CSharpLibrary;

public class Interop
{
    [UnmanagedCallersOnly(EntryPoint = "add")]
    public static int Add(int x, int y) => x + y;
}
