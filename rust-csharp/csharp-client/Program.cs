using System.Runtime.InteropServices;

var result = RustLibrary.Add_DllImport(1, 2);
Console.WriteLine($"1 + 2 = {result}");

result = RustLibrary.Add(1, 2);
Console.WriteLine($"1 + 2 = {result}");

var resultVector = RustLibrary.AddVectors(new Vector(1f, 2f), new Vector(3f, 4f));
Console.WriteLine($"(1, 2) + (3, 4) = ({resultVector.X}, {resultVector.Y})");

Console.WriteLine($"The text is {RustLibrary.StrLen("Hello 👋")} characters long");

static partial class RustLibrary
{
    private const string RustLib = "librust_library";

    // DllImport is the "classical" way to import an unmanaged library.
    // Read more: https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke
    [DllImport(RustLib, EntryPoint = "add")]
    public static extern int Add_DllImport(int x, int y);

    // .NET 7 introduced a new way to import unmanaged libraries. It generates
    // the code required to marshal data between managed and unmanaged code at
    // compile time. This is now the recommended way to import unmanaged libraries.
    [LibraryImport(RustLib, EntryPoint = "add")]
    public static partial int Add(int x, int y);

    [LibraryImport(RustLib, EntryPoint = "add_vec2d")]
    public static partial Vector AddVectors(Vector x, Vector y);

    [LibraryImport(RustLib, EntryPoint = "strlen", StringMarshalling = StringMarshalling.Utf8)]
    public static partial uint StrLen(string str);

    [LibraryImport(RustLib, EntryPoint = "to_upper", StringMarshalling = StringMarshalling.Utf8)]
    public static unsafe partial void ToUpper(string str, byte* buffer);
}

// The struct must be marked as sequential to ensure that the fields are laid
// out in memory in the same order as they are declared in the source code.
// Read more: https://learn.microsoft.com/en-us/dotnet/standard/native-interop/type-marshalling#marshalling-classes-and-structs
[StructLayout(LayoutKind.Sequential)]
record struct Vector(float X, float Y);
