import 'dart:ffi';
import 'dart:io';
import 'bridge_generated.dart';

const base = 'native';
// final path = Platform.isWindows ? '$base.dll' : Platform.isMacOS ? "lib$base.dylib" : 'lib$base.so';
// late final dylib = loadLibForFlutter(path);
final dylib = Platform.isWindows
    ? DynamicLibrary.open('$base.dll')
    : Platform.isAndroid
        ? DynamicLibrary.open('lib$base.so')
        : DynamicLibrary.executable();
late final api = NativeImpl(dylib);
