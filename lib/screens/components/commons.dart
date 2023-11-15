
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_styled_toast/flutter_styled_toast.dart';
import 'package:permission_handler/permission_handler.dart';

import '../../cross.dart';

/// 显示一个toast
void defaultToast(BuildContext context, String title, {int seconds = 2}) {
  showToast(
    title,
    context: context,
    position: StyledToastPosition.center,
    animation: StyledToastAnimation.scale,
    reverseAnimation: StyledToastAnimation.fade,
    duration: Duration(seconds: seconds),
    animDuration: const Duration(seconds: 0, milliseconds: 800),
    curve: Curves.elasticOut,
    reverseCurve: Curves.linear,
  );
}

var _controller =
TextEditingController.fromValue(const TextEditingValue(text: ''));

Future<String?> displayTextInputDialog(BuildContext context,
    {String? title,
      String src = "",
      String? hint,
      String? desc,
      bool isPasswd = false}) {
  _controller.text = src;
  return showDialog(
    context: context,
    builder: (context) {
      return AlertDialog(
        title: title == null ? null : Text(title),
        content: SingleChildScrollView(
          child: ListBody(
            children: [
              TextField(
                controller: _controller,
                decoration: InputDecoration(hintText: hint),
                obscureText: isPasswd,
                obscuringCharacter: '\u2022',
              ),
              ...(desc == null
                  ? []
                  : [
                Container(
                  padding: const EdgeInsets.only(top: 20, bottom: 10),
                  child: Text(
                    desc,
                    style: TextStyle(
                        fontSize: 12,
                        color: Theme.of(context)
                            .textTheme
                            .bodyText1
                            ?.color
                            ?.withOpacity(.5)),
                  ),
                )
              ]),
            ],
          ),
        ),
        actions: <Widget>[
          MaterialButton(
            child: const Text('取消'),
            onPressed: () {
              Navigator.of(context).pop();
            },
          ),
          MaterialButton(
            child: const Text('确认'),
            onPressed: () {
              Navigator.of(context).pop(_controller.text);
            },
          ),
        ],
      );
    },
  );
}

Future<T?> chooseListDialog<T>(BuildContext context,
    {required List<T> values, required String title, String? tips}) async {
  return showDialog<T>(
    context: context,
    builder: (BuildContext context) {
      return SimpleDialog(
        title: Text(title),
        children: [
          ...values.map((e) => SimpleDialogOption(
            onPressed: () {
              Navigator.of(context).pop(e);
            },
            child: Text('$e'),
          )),
          ...tips != null
              ? [
            Container(
              padding: const EdgeInsets.fromLTRB(15, 5, 15, 15),
              child: Text(tips),
            ),
          ]
              : [],
        ],
      );
    },
  );
}

Future saveImageFileToGallery(BuildContext context, String path) async {
  if (Platform.isAndroid) {
    if (!(await Permission.storage.request()).isGranted) {
      return;
    }
  }
  if (Platform.isIOS || Platform.isAndroid) {
    await cross.saveImageToGallery(path);
    defaultToast(context, "保存成功");
    return;
  }
  defaultToast(context, "暂不支持该平台");
}

Future<T?> chooseMapDialog<T>(
    BuildContext buildContext, {
      required String title,
      required Map<String, T> values,
    }) async {
  return await showDialog<T>(
    context: buildContext,
    builder: (BuildContext context) {
      return SimpleDialog(
        title: Text(title),
        children: values.entries
            .map((e) => SimpleDialogOption(
          child: Text(e.key),
          onPressed: () {
            Navigator.of(context).pop(e.value);
          },
        ))
            .toList(),
      );
    },
  );
}
