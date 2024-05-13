import 'package:event/event.dart';
import 'package:flutter/cupertino.dart';
import 'package:kobi/screens/components/commons.dart';
import '../src/rust/api/api.dart' as api;
import '../src/rust/udto.dart';

bool _logging = true;

bool get logging => _logging;

final loginEvent = Event<EventArgs>();

UILoginState _loginState = const UILoginState(
  state: 0,
  message: "",
  member: null,
);

UILoginState get loginState => _loginState;

Future initLogin() async {
  _logging = true;
  loginEvent.broadcast();
  _loginState = await api.initLoginState();
  _logging = false;
  loginEvent.broadcast();
}

Future login(String username, String password) async {
  _logging = true;
  loginEvent.broadcast();
  _loginState = await api.login(username: username, password: password);
  _logging = false;
  loginEvent.broadcast();
}

Future register(BuildContext context, String username, String password) async {
  _logging = true;
  loginEvent.broadcast();
  final result = await api.register(username: username, password: password);
  if (result.state == 1) {
    defaultToast(context, "注册成功, 请登录", seconds: 10);
  } else {
    defaultToast(context, result.message, seconds: 10);
  }
  _logging = false;
  loginEvent.broadcast();
}
