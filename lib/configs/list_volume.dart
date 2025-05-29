import 'package:flutter/material.dart';

import '../../src/rust/api/api.dart' as api;
import '../../src/rust/udto.dart';
import '../screens/components/commons.dart';

const _propertyName = "listVolume";
late bool _listVolume;

Future initListVolume() async {
  _listVolume = false;
  final st = await api.loadProperty(k: _propertyName);
  if (st.isNotEmpty) {
    try {
      _listVolume = bool.parse(st);
    } catch (e) {}
  }
}

bool get currentListVolume => _listVolume;

Future chooseListVolume(BuildContext context) async {
  final Map<String, bool> map = {};
  map["是"] = true;
  map["否"] = false;
  final newListVolume = await chooseMapDialog(
    context,
    title: "是否启动音量翻页",
    values: map,
  );
  if (newListVolume != null) {
    await api.saveProperty(k: _propertyName, v: "$newListVolume");
    _listVolume = newListVolume;
  }
}

Widget listVolumeSwitch() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return SwitchListTile(
        title: const Text("启动音量翻页"),
        value: currentListVolume,
        onChanged: (value) async {
          await api.saveProperty(k: _propertyName, v: "$value");
          setState(() {
            _listVolume = value;
          });
        },
      );
    },
  );
}
