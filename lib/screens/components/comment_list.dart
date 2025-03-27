import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:kobi/configs/login.dart';
import 'package:kobi/screens/components/commnet_card.dart';
import 'package:kobi/screens/components/commons.dart';
import 'package:kobi/screens/components/content_loading.dart';
import 'package:kobi/src/rust/api/api.dart' as api;
import 'package:kobi/src/rust/udto.dart';

import 'content_error.dart';

class CommnetList extends StatefulWidget {
  final String comicId;
  final int? parentId;

  const CommnetList(this.comicId, {Key? key, this.parentId}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _CommnetListState();
}

class _CommnetListState extends State<CommnetList> {
  late int _commentOffset;
  late Future<UIPageComment> _commentFuture;

  @override
  void initState() {
    _load(0);
    loginEvent.subscribe(_setState);
    super.initState();
  }

  @override
  void dispose() {
    loginEvent.unsubscribe(_setState);
    super.dispose();
  }

  _setState(_) {
    setState(() {});
  }

  _load(int offser) {
    setState(() {
      _commentOffset = offser;
      _commentFuture = api.comments(
        comicId: widget.comicId,
        offset: BigInt.from(_commentOffset),
        limit: BigInt.from(10),
        replyId: widget.parentId?.toString() ?? "",
      );
    });
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: _commentFuture,
      builder: (BuildContext context, AsyncSnapshot<UIPageComment> snapshot) {
        if (snapshot.hasError) {
          return ContentError(
            onRefresh: () async {
              setState(() {
                _commentFuture = api.comments(
                    comicId: widget.comicId,
                    offset: BigInt.from(_commentOffset),
                    limit: BigInt.from(10));
              });
            },
            error: snapshot.error,
            stackTrace: snapshot.stackTrace,
            sq: true,
          );
        }
        if (snapshot.connectionState != ConnectionState.done) {
          return ContentLoading(sq: true);
        }
        var data = snapshot.requireData;
        return Column(
          children: [
            if (widget.parentId == null && loginState.state == 1)
              GestureDetector(
                onTap: () async {
                  var comment = await displayTextInputDialog(context);
                  if (comment != null && comment.isNotEmpty) {
                    try {
                      await api.sendComment(
                        comicId: widget.comicId,
                        comment: comment,
                      );
                      defaultToast(context, "发送成功");
                    } catch (e) {
                      if (e is AnyhowException) {
                        defaultToast(context, "发送失败 : ${e.message}");
                      } else {
                        defaultToast(context, "发送失败 : $e");
                      }
                      debugPrint("$e");
                    }
                  }
                },
                child: Container(
                  padding: const EdgeInsets.only(top: 20, bottom: 20),
                  child: Text(
                    "我有话要讲",
                    style: TextStyle(
                        fontSize: 12,
                        color: Theme.of(context)
                            .textTheme
                            .bodyMedium
                            ?.color
                            ?.withOpacity(.5)),
                  ),
                ),
              ),
            GestureDetector(
              onTap: () {
                _load(_commentOffset);
              },
              child: Container(
                padding: const EdgeInsets.only(top: 20, bottom: 20),
                child: Text(
                  "刷新",
                  style: TextStyle(
                      fontSize: 12,
                      color: Theme.of(context)
                          .textTheme
                          .bodyMedium
                          ?.color
                          ?.withOpacity(.5)),
                ),
              ),
            ),
            if (data.offset > 0)
              TextButton(
                onPressed: () {
                  _load(data.offset - data.limit);
                },
                child: const Text('上一页'),
              ),
            ...data.list.map((e) {
              return GestureDetector(
                onTap: () {
                  if (widget.parentId != null) {
                    return;
                  }
                  Navigator.of(context)
                      .push(MaterialPageRoute(builder: (context) {
                    return Scaffold(
                      appBar: AppBar(
                        title: const Text('评论'),
                        actions: [
                          ReplyButton(
                              comicId: widget.comicId,
                              commentId: e.id.toString()),
                        ],
                      ),
                      body: Column(
                        children: [
                          CommentCard(e),
                          Expanded(
                            child: ListView(
                              children: [
                                CommnetList(widget.comicId, parentId: e.id),
                              ],
                            ),
                          ),
                        ],
                      ),
                    );
                  }));
                },
                child: CommentCard(e),
              );
            }),
            if (data.offset + data.limit < data.total)
              TextButton(
                onPressed: () {
                  _load(data.offset + data.limit);
                },
                child: const Text('下一页'),
              ),
          ],
        );
      },
    );
  }
}

class ReplyButton extends StatefulWidget {
  final String comicId;
  final String commentId;

  const ReplyButton({Key? key, required this.comicId, required this.commentId})
      : super(key: key);

  @override
  State<StatefulWidget> createState() => _ReplyButtonState();
}

class _ReplyButtonState extends State<ReplyButton> {
  @override
  void initState() {
    loginEvent.subscribe(_setState);
    super.initState();
  }

  @override
  void dispose() {
    loginEvent.unsubscribe(_setState);
    super.dispose();
  }

  _setState(_) {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    if (loginState.state != 1) {
      return const SizedBox();
    }
    return IconButton(
      onPressed: () async {
        var comment = await displayTextInputDialog(context);
        if (comment != null && comment.isNotEmpty) {
          try {
            await api.sendComment(
              comicId: widget.comicId,
              comment: comment,
              replyId: widget.commentId,
            );
            defaultToast(context, "发送成功");
          } catch (e) {
            if (e is AnyhowException) {
              defaultToast(context, "发送失败 : ${e.message}");
            } else {
              defaultToast(context, "发送失败 : $e");
            }
            debugPrint("$e");
          }
        }
      },
      icon: const Icon(Icons.reply),
    );
  }
}
