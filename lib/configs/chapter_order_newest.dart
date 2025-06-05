import 'package:flutter/material.dart';

import '../../src/rust/api/api.dart' as api;
import '../../src/rust/udto.dart';
import '../screens/components/commons.dart';

const _propertyName = "chapterOrderNewest";
late bool _chapterOrderNewest;

Future initChapterOrderNewest() async {
  _chapterOrderNewest = false;
  final st = await api.loadProperty(k: _propertyName);
  if (st.isNotEmpty) {
    try {
      _chapterOrderNewest = bool.parse(st);
    } catch (e) {}
  }
}

bool get currentChapterOrderNewest => _chapterOrderNewest;

Future chooseChapterOrderNewest(BuildContext context) async {
  final Map<String, bool> map = {};
  map["是"] = true;
  map["否"] = false;
  final newChapterOrderNewest = await chooseMapDialog(
    context,
    title: "详情展示章节排序反转",
    values: map,
  );
  if (newChapterOrderNewest != null) {
    await api.saveProperty(k: _propertyName, v: "$newChapterOrderNewest");
    _chapterOrderNewest = newChapterOrderNewest;
  }
}

Widget chapterOrderNewestSwitch() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return SwitchListTile(
        title: const Text("详情展示章节排序反转"),
        value: currentChapterOrderNewest,
        onChanged: (value) async {
          await api.saveProperty(k: _propertyName, v: "$value");
          setState(() {
            _chapterOrderNewest = value;
          });
        },
      );
    },
  );
}
