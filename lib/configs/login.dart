import 'package:event/event.dart';
import '../bridge_generated.dart';
import '../ffi.io.dart';

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
  _loginState = await api.initLoginState();
  _logging = false;
  loginEvent.broadcast();
}
