import 'package:flutter/material.dart';
import 'package:kobi/configs/login.dart';
import 'package:kobi/screens/settings_screen.dart';

class UserScreen extends StatelessWidget {
  const UserScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('UserScreen'),
      ),
      body: ListView(
        children: [
          const UserInfoCard(),
          ListTile(
            title: const Text('设置'),
            onTap: () {
              Navigator.of(context).push(MaterialPageRoute(builder: (context) {
                return const SettingsScreen();
              }));
            },
          ),
        ],
      ),
    );
  }
}

class UserInfoCard extends StatefulWidget {
  const UserInfoCard({super.key});

  @override
  State<StatefulWidget> createState() {
    return _UserInfoCardState();
  }
}

class _UserInfoCardState extends State<UserInfoCard> {
  @override
  void initState() {
    super.initState();
    loginEvent.subscribe(_setState);
  }

  @override
  void dispose() {
    loginEvent.unsubscribe(_setState);
    super.dispose();
  }

  void _setState(dynamic a) {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 140,
      padding: const EdgeInsets.all(10),
      decoration: BoxDecoration(
        border: Border.all(
          width: 0.5,
          color: Colors.grey.withOpacity(.5),
          style: BorderStyle.solid,
        ),
      ),
      child: _buildCard(),
    );
  }

  Widget _buildCard() {
    return Column(
      children: [
        Expanded(child: Container()),
        _buildAvatar(),
        Expanded(child: Container()),
        _buildName(),
        Expanded(child: Container()),
      ],
    );
  }

  Widget _buildAvatar() {
    late IconData id;
    if (logging) {
      id = Icons.refresh;
    } else if (loginState.state == 0) {
      id = Icons.no_accounts_sharp;
    } else if (loginState.state == 1) {
      id = Icons.face;
    } else if (loginState.state == 2) {
      id = Icons.error_outline;
    } else {
      throw Exception('Unknown loginState.state: ${loginState.state}');
    }
    return Icon(
      id,
      size: 50,
    );
  }

  Widget _buildName() {
    late String name;
    if (logging) {
      name = "登录中";
    } else if (loginState.state == 0) {
      name = "未登录";
    } else if (loginState.state == 1) {
      name = loginState.member!.nickname;
    } else if (loginState.state == 2) {
      name = "登录失败";
    } else {
      throw Exception('Unknown loginState.state: ${loginState.state}');
    }
    return Text(name);
  }
}
