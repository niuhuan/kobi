import 'package:flutter/material.dart';
import 'package:event/event.dart';
import '../src/rust/api/api.dart' as api;

enum ComicPagerType {
  grid, // 多列网格
  list, // 详情列表
}

const _propertyName = "comic_pager_type";

late ComicPagerType _comicPagerType = ComicPagerType.list;

ComicPagerType get currentComicPagerType => _comicPagerType;

class ComicPagerTypeEventArgs extends EventArgs {
  final ComicPagerType type;
  ComicPagerTypeEventArgs(this.type);
}

final Event<ComicPagerTypeEventArgs> comicPagerTypeEvent = Event<ComicPagerTypeEventArgs>();

Future initComicPagerType() async {
  var value = await api.loadProperty(k: _propertyName);
  if (value == null) {
    await api.saveProperty(k: _propertyName, v: ComicPagerType.list.name);
    _comicPagerType = ComicPagerType.list;
  } else {
    _comicPagerType = ComicPagerType.values.firstWhere(
      (e) => e.name == value,
      orElse: () => ComicPagerType.list,
    );
  }
  comicPagerTypeEvent.broadcast(ComicPagerTypeEventArgs(_comicPagerType));
}

Future chooseComicPagerType(BuildContext context) async {
  var result = await showDialog<ComicPagerType>(
    context: context,
    builder: (BuildContext context) {
      return AlertDialog(
        backgroundColor: const Color(0xAA000000),
        title: const Text(
          "选择漫画列表显示方式",
          style: TextStyle(color: Colors.white),
        ),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              title: const Text(
                "多列网格",
                style: TextStyle(color: Colors.white),
              ),
              onTap: () {
                Navigator.of(context).pop(ComicPagerType.grid);
              },
            ),
            ListTile(
              title: const Text(
                "详情列表",
                style: TextStyle(color: Colors.white),
              ),
              onTap: () {
                Navigator.of(context).pop(ComicPagerType.list);
              },
            ),
          ],
        ),
      );
    },
  );
  if (result != null) {
    await api.saveProperty(k: _propertyName, v: result.name);
    _comicPagerType = result;
    comicPagerTypeEvent.broadcast(ComicPagerTypeEventArgs(_comicPagerType));
  }
}

String comicPagerTypeName(ComicPagerType type, BuildContext context) {
  switch (type) {
    case ComicPagerType.grid:
      return "多列网格";
    case ComicPagerType.list:
      return "详情列表";
  }
}

Widget comicPagerTypeSetting(BuildContext context) {
  return ListTile(
    title: const Text(
      "漫画列表显示方式",
      style: TextStyle(color: Colors.white),
    ),
    subtitle: Text(
      comicPagerTypeName(currentComicPagerType, context),
      style: const TextStyle(color: Colors.white70),
    ),
    onTap: () => chooseComicPagerType(context),
  );
} 