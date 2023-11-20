import 'package:flutter/material.dart';
import 'package:kobi/configs/login.dart';
import 'package:kobi/screens/collected_comics_account_screen.dart';
import 'package:kobi/screens/components/commons.dart';
import 'package:kobi/screens/downloads_screen.dart';
import 'package:kobi/screens/histories_screen.dart';
import 'package:kobi/screens/settings_screen.dart';

import 'local_collect_screen.dart';
import 'login_screen.dart';

class UserScreen extends StatelessWidget {
  const UserScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('偏好'),
        actions: [
          IconButton(
            onPressed: () {
              Navigator.of(context).push(MaterialPageRoute(builder: (context) {
                return const SettingsScreen();
              }));
            },
            icon: const Icon(Icons.settings),
          )
        ],
      ),
      body: ListView(
        children: [
          UserInfoCard(),
          Container(
            height: 50,
          ),
          Divider(),
          ListTile(
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                    builder: (context) => const HistoriesScreen()),
              );
            },
            title: const Text('历史记录(本地)'),
          ),
          Divider(),
          ListTile(
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                    builder: (context) => const DownloadsScreen()),
              );
            },
            title: const Text('下载列表(本地)'),
          ),
          Divider(),
          ListTile(
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                    builder: (context) => const CollectedComicsAccountScreen()),
              );
            },
            title: const Text('收藏列表(账户)'),
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
    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: _onTap,
      child: Container(
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
      ),
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

  _onTap() async {
    if (logging) {
      defaultToast(context, "登录中，请稍后");
      return;
    }
    if (loginState.state == 0 || loginState.state == 2) {
      Navigator.push(
        context,
        MaterialPageRoute(builder: (context) => const LoginScreen()),
      );
    }
  }
}
