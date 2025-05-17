import 'package:flutter/material.dart';

import '../../src/rust/api/api.dart' as api;
import '../../src/rust/udto.dart';
import '../screens/components/commons.dart';

const _propertyName = "noPagerAnimation";
late bool _noPagerAnimation;

Future initNoPagerAnimation() async {
  _noPagerAnimation = false;
  final st = await api.loadProperty(k: _propertyName);
  if (st.isNotEmpty) {
    try {
      _noPagerAnimation = bool.parse(st);
    } catch (e) {}
  }
}

bool get currentNoPagerAnimation => _noPagerAnimation;

Future chooseNoPagerAnimation(BuildContext context) async {
  final Map<String, bool> map = {};
  map["是"] = true;
  map["否"] = false;
  final newNoPagerAnimation = await chooseMapDialog(
    context,
    title: "是否禁用翻页动画",
    values: map,
  );
  if (newNoPagerAnimation != null) {
    await api.saveProperty(k: _propertyName, v: "$newNoPagerAnimation");
    _noPagerAnimation = newNoPagerAnimation;
  }
}

Widget noPagerAnimationSwitch() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return SwitchListTile(
        title: const Text("禁用翻页动画"),
        value: currentNoPagerAnimation,
        onChanged: (value) {
          setState(() {
            _noPagerAnimation = value;
          });
        },
      );
    },
  );
}
