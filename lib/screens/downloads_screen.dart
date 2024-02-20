import 'package:flutter/material.dart';

import '../bridge_generated.dart';
import '../ffi.io.dart';
import 'components/commons.dart';
import 'components/download_comic_card.dart';
import 'download_comic_info_screen.dart';

class DownloadsScreen extends StatefulWidget {
  const DownloadsScreen({super.key});

  @override
  State<StatefulWidget> createState() => _DownloadsScreenState();
}

class _DownloadsScreenState extends State<DownloadsScreen> {
  List<UIDownloadComic> list = [];
  bool paused = false;

  _init() async {
    list = await api.downloadComics();
    paused = await api.downloadIsPause();
    setState(() {});
  }

  @override
  void initState() {
    _init();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    final pager = Column(
      children: [
        Container(
          padding: const EdgeInsets.only(
            left: 10,
            right: 10,
          ),
          decoration: BoxDecoration(
            border: Border(
              bottom: BorderSide(
                color: Colors.grey.shade400,
                width: .5,
              ),
            ),
          ),
          child: Row(
            children: [
              Expanded(child: Container()),
              Text(paused ? "暂停中" : "下载中"),
              IconButton(
                onPressed: () async {
                  await api.downloadSetPause(pause: !paused);
                  await _init();
                },
                icon: Icon(paused ? Icons.play_arrow : Icons.pause),
              ),
              IconButton(
                onPressed: () async {
                  await api.resetFailDownloads();
                  defaultToast(context, "失败的任务已经重置");
                  await _init();
                },
                icon: const Icon(Icons.refresh),
              ),
            ],
          ),
        ),
        Expanded(
          child: ListView(
            padding: EdgeInsets.zero,
            children: [
              ...list.map((e) => GestureDetector(
                    onTap: () async {
                      Navigator.of(context).push(
                        MaterialPageRoute(
                          builder: (context) => DownloadComicInfoScreen(e),
                        ),
                      );
                    },
                    onLongPress: () async {
                      // confirm delete
                      final ok = await showDialog<bool>(
                        context: context,
                        builder: (context) => AlertDialog(
                          title: const Text("确认删除"),
                          content: const Text("删除后将无法恢复"),
                          actions: [
                            TextButton(
                              onPressed: () {
                                Navigator.of(context).pop(false);
                              },
                              child: const Text("取消"),
                            ),
                            TextButton(
                              onPressed: () {
                                Navigator.of(context).pop(true);
                              },
                              child: const Text("确认"),
                            ),
                          ],
                        ),
                      );
                      if (ok == true) {
                        await api.deleteDownloadComic(
                            comicPathWord: e.pathWord);
                        await _init();
                      }
                    },
                    child: DownloadComicCard(e),
                  )),
            ],
          ),
        ),
      ],
    );
    return Scaffold(
      appBar: AppBar(
        title: const Text("下载管理"),
      ),
      body: pager,
    );
  }
}
