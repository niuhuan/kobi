import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import '../../src/rust/api/api.dart' as api;
import '../../src/rust/udto.dart';
import 'package:kobi/screens/components/images.dart';

class ImageCacheProvider extends ImageProvider<ImageCacheProvider> {
  final String url;
  final String useful;
  final double scale;
  final String? extendsFieldFirst;
  final String? extendsFieldSecond;
  final String? extendsFieldThird;

  ImageCacheProvider({
    required this.url,
    required this.useful,
    this.extendsFieldFirst,
    this.extendsFieldSecond,
    this.extendsFieldThird,
    this.scale = 1.0,
  });

  @override
  ImageStreamCompleter loadImage(
      ImageCacheProvider key, ImageDecoderCallback decode) {
    return MultiFrameImageStreamCompleter(
      codec: _loadAsync(key),
      scale: key.scale,
    );
  }

  @override
  Future<ImageCacheProvider> obtainKey(ImageConfiguration configuration) {
    return SynchronousFuture<ImageCacheProvider>(this);
  }

  Future<ui.Codec> _loadAsync(ImageCacheProvider key) async {
    assert(key == this);
    final path = (await api.cacheImage(
      cacheKey: imageUrlToCacheKey(url),
      url: url,
      useful: useful,
      extendsFieldFirst: extendsFieldFirst,
      extendsFieldSecond: extendsFieldSecond,
      extendsFieldThird: extendsFieldThird,
    ))
        .absPath;
    return ui.instantiateImageCodec(
      await _loadImageFile(path),
    );
  }

  @override
  bool operator ==(dynamic other) {
    if (other.runtimeType != runtimeType) return false;
    final ImageCacheProvider typedOther = other;
    return url == typedOther.url && scale == typedOther.scale;
  }

  @override
  int get hashCode => hashValues(url, scale);

  @override
  String toString() => '$runtimeType('
      'path: ${describeIdentity(url)},'
      ' scale: $scale'
      ')';
}

Future<Uint8List> _loadImageFile(String path) {
  return File(path).readAsBytes();
}
