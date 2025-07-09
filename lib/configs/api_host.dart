/// 代理设置

import 'package:flutter/material.dart';
import 'package:kobi/src/rust/copy_client/client.dart';

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
      return Column(
        children: [
          ListTile(
            title: const Text("服务器地址"),
            subtitle: Text(currentApiHostName()),
            onTap: () async {
              await inputApiHost(context);
              setState(() {});
            },
          ),
          ListTile(
            title: const Text("同步服务器和请求头"),
            onTap: () async {
              try {
                String apiHost = await api.syncApiHost();
                _currentApiHost = apiHost;
                defaultToast(context, "同步成功");
              } catch (e, s) {
                print(e);
                print(s);
                defaultToast(context, "同步失败");
              }
              setState(() {});
            },
          ),
          ListTile(
            title: const Text("查看现在的请求头"),
            onTap: () async {
              List<CopyHeader> headers = await api.getAllHeaders();
              showDialog(
                context: context,
                builder: (context) => AlertDialog(
                  title: const Text("请求头"),
                  content: Text(
                      headers.map((e) => "${e.key}: ${e.value}").join("\n")),
                ),
              );
            },
          ),
        ],
      );
    },
  );
}
