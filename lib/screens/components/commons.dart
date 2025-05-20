
import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_styled_toast/flutter_styled_toast.dart';
import 'package:kobi/screens/components/android_version.dart';
import 'package:kobi/src/rust/api/api.dart';
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
                            .bodyMedium
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
  Future? future;
  if (Platform.isIOS) {
    await cross.saveImageToGallery(path);
  } else if (Platform.isAndroid) {
    bool g;
    // if (androidVersion < 30) {
    //   g = await Permission.storage.request().isGranted;
    // } else {
    //   g = await Permission.manageExternalStorage.request().isGranted;
    // }
    // if (!g) {
    //   return;
    // }
    await cross.saveImageToGallery(path);
  } else if (Platform.isWindows || Platform.isMacOS || Platform.isLinux) {
    defaultToast(context, '暂不支持该平台');
    // String? folder = await chooseFolder(context);
    // if (folder != null) {
    //   future = method.convertImageToJPEG100(path, folder);
    // }
  } else {
    defaultToast(context, '暂不支持该平台');
    return;
  }
  if (future == null) {
    defaultToast(context, '保存取消');
    return;
  }
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
