/// 代理设置

import 'package:flutter/material.dart';

import '../src/rust/api/api.dart' as api;
import '../screens/components/commons.dart';

late String _currentApiHost;

Future<String?> initApiHost() async {
  _currentApiHost = await api.getApiHost();
  return null;
}

String currentApiHostName() {
  return _currentApiHost == "" ? "未设置" : _currentApiHost;
}

Future<dynamic> inputApiHost(BuildContext context) async {
  String? input = await displayTextInputDialog(
    context,
    src: _currentApiHost,
    title: '服务器',
    hint: '请输入服务器',
    desc: " ( 例如 https://domain.com ) ",
  );
  if (input != null) {
    await api.setApiHost(api: input);
    _currentApiHost = input;
  }
}

Widget apiHostSetting() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: const Text("服务器地址"),
        subtitle: Text(currentApiHostName()),
        onTap: () async {
          await inputApiHost(context);
          setState(() {});
        },
      );
    },
  );
}
