
import 'package:flutter/material.dart';
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