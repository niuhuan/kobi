import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';
import '../../ffi.io.dart';
import 'commons.dart';
import 'file_photo_view_screen.dart';

String imageUrlToCacheKey(String url){
  final uri = Uri.parse(url);
  return uri.path;
}

final errorColor = Color.alphaBlend(Colors.red.withOpacity(.2), Colors.black12);

Widget buildError(double? width, double? height) {
  double? size;
  if (width != null && height != null) {
    size = width < height ? width : height;
  }
  return SizedBox(
    width: width,
    height: height,
    child: Center(
      child: Icon(
        Icons.error_outline,
        size: size,
        color: errorColor,
      ),
    ),
  );
}

Widget buildLoading(double? width, double? height) {
  double? size;
  if (width != null && height != null) {
    size = width < height ? width : height;
  }
  return SizedBox(
    width: width,
    height: height,
    child: Center(
      child: Icon(
        Icons.downloading,
        size: size,
        color: Colors.black12,
      ),
    ),
  );
}

//
class LoadingCacheImage extends StatefulWidget {
  final String url;
  final String useful;
  final String? extendsFieldFirst;
  final String? extendsFieldSecond;
  final String? extendsFieldThird;
  final double? width;
  final double? height;
  final Function(Size size)? onTrueSize;
  final BoxFit fit;

  const LoadingCacheImage({
    Key? key,
    required this.url,
    required this.useful,
    this.extendsFieldFirst,
    this.extendsFieldSecond,
    this.extendsFieldThird,
    this.width,
    this.height,
    this.onTrueSize,
    this.fit = BoxFit.cover,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _LoadingCacheImageState();
}

class _LoadingCacheImageState extends State<LoadingCacheImage> {
  late Future<String> _future;

  @override
  void initState() {
    _future = _init();
    super.initState();
  }

  Future<String> _init() async {
    final cacheKey = imageUrlToCacheKey(widget.url);
    final loadedImage = await api.cacheImage(
      cacheKey: cacheKey,
      url: widget.url,
      useful: widget.useful,
      extendsFieldFirst: widget.extendsFieldFirst,
      extendsFieldSecond: widget.extendsFieldSecond,
      extendsFieldThird: widget.extendsFieldThird,
    );
    widget.onTrueSize?.call(Size(
        loadedImage.imageWidth.toDouble(), loadedImage.imageHeight.toDouble()));
    return loadedImage.absPath;
  }

  @override
  Widget build(BuildContext context) {
    return pathFutureImage(
      _future,
      widget.width,
      widget.height,
      fit: widget.fit,
    );
  }
}

Widget pathFutureImage(Future<String> future, double? width, double? height,
    {BoxFit fit = BoxFit.cover, BuildContext? context}) {
  return FutureBuilder(
      future: future,
      builder: (BuildContext context, AsyncSnapshot<String> snapshot) {
        if (snapshot.hasError) {
          print("${snapshot.error}");
          print("${snapshot.stackTrace}");
          return buildError(width, height);
        }
        if (snapshot.connectionState != ConnectionState.done) {
          return buildLoading(width, height);
        }
        return buildFile(
          snapshot.data!,
          width,
          height,
          fit: fit,
          context: context,
        );
      });
}

// 通用方法

Widget buildSvg(String source, double? width, double? height,
    {Color? color, double? margin}) {
  var widget = Container(
    width: width,
    height: height,
    padding: margin != null ? const EdgeInsets.all(10) : null,
    child: Center(
      child: SvgPicture.asset(
        source,
        width: width,
        height: height,
        color: color,
      ),
    ),
  );
  return GestureDetector(onLongPress: () {}, child: widget);
}

Widget buildFile(String file, double? width, double? height,
    {BoxFit fit = BoxFit.cover, BuildContext? context}) {
  var image = Image(
    image: FileImage(File(file)),
    width: width,
    height: height,
    errorBuilder: (a, b, c) {
      print("$b");
      print("$c");
      return buildError(width, height);
    },
    fit: fit,
  );
  if (context == null) return image;
  return GestureDetector(
    onLongPress: () async {
      String? choose = await chooseListDialog(
        context,
        title: '请选择',
        values: ['预览图片', '保存图片'],
      );
      switch (choose) {
        case '预览图片':
          Navigator.of(context).push(MaterialPageRoute(
            builder: (context) => FilePhotoViewScreen(file),
          ));
          break;
        case '保存图片':
          saveImageFileToGallery(context, file);
          break;
      }
    },
    child: image,
  );
}
