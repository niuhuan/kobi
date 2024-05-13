import 'package:flutter/material.dart';

import '../src/rust/api/api.dart' as api;
import '../src/rust/udto.dart';
import '../screens/components/commons.dart';

const _propertyKey = "cache_time";

const _sec_of_day = 3600 * 24;
const _src_of_3_day = _sec_of_day * 3;
const _src_of_week = _sec_of_day * 7;

const _sec_of_day_str = "1天";
const _src_of_3_day_str = "3天";
const _src_of_week_str = "1周";

const _nameValueMap = {
  _sec_of_day_str: _sec_of_day,
  _src_of_3_day_str: _src_of_3_day,
  _src_of_week_str: _src_of_week,
};

const _valueNameMap = {
  _sec_of_day: _sec_of_day_str,
  _src_of_3_day: _src_of_3_day_str,
  _src_of_week: _src_of_week_str,
};

int _value = 0;

Future initCacheTime() async {
  final time = await api.loadProperty(k: _propertyKey);
  if (time.isEmpty) {
    _value = _src_of_week;
  } else {
    _value = int.parse(time);
  }
  if (_value > 0) {
    await api.cleanCache(time: _value);
  }
}

String cacheTimeName(BuildContext context) {
  if (_value == 0) {
    return "不清理";
  }
  String? name = _valueNameMap[_value];
  if (name != null) {
    return name;
  }
  return "$_value SEC";
}

Future chooseAutoClean(BuildContext context) async {
  int? choose = await chooseMapDialog(context,
      title: "缓存保留时间",
      values: _nameValueMap.map((key, value) => MapEntry(key, value)));
  if (choose != null) {
    await api.saveProperty(k: _propertyKey, v: "$choose");
    _value = choose;
  }
}

Widget cacheTimeNameSetting() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: const Text("缓存保留时间"),
        subtitle: Text(cacheTimeName(context)),
        onTap: () async {
          await chooseAutoClean(context);
          setState(() {});
        },
      );
    },
  );
}
