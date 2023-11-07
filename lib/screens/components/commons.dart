
import 'package:flutter/material.dart';
import 'package:flutter_styled_toast/flutter_styled_toast.dart';

/// 显示一个toast
void defaultToast(BuildContext context, String title) {
  showToast(
    title,
    context: context,
    position: StyledToastPosition.center,
    animation: StyledToastAnimation.scale,
    reverseAnimation: StyledToastAnimation.fade,
    duration: const Duration(seconds: 2),
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
