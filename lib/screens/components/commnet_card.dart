import 'package:flutter/material.dart';
import 'package:kobi/src/rust/udto.dart';

import 'images.dart';

class CommentCard extends StatelessWidget {
  final UIComment comment;

  const CommentCard(this.comment, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(10),
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(
            color: Colors.grey.shade400,
            width: .5,
          ),
        ),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              // https://hi77-overseas.mangafuna.xyz/user/cover/copymanga.png
              if (comment.userAvatar.isEmpty ||
                  comment.userAvatar.endsWith('copymanga.png'))
                const ClipRRect(
                  borderRadius: BorderRadius.all(Radius.circular(30)),
                  child: Icon(
                    Icons.account_circle,
                    size: 30,
                    color: Colors.grey,
                  ),
                )
              else
                ClipRRect(
                  borderRadius: const BorderRadius.all(Radius.circular(30)),
                  child: LoadingCacheImage(
                    url: comment.userAvatar,
                    width: 30,
                    height: 30,
                    useful: 'USER_AVATAR',
                    extendsFieldFirst: comment.userId,
                    fit: BoxFit.cover,
                  ),
                ),
              Container(
                width: 10,
              ),
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    comment.userName,
                    style: const TextStyle(
                      fontSize: 14,
                    ),
                  ),
                  Text(
                    comment.createAt,
                    style: const TextStyle(
                      fontSize: 12,
                      color: Colors.grey,
                    ),
                  ),
                ],
              ),
              Expanded(child: Container()),
              Column(
                children: [
                  Row(
                    children: [
                      const Icon(
                        Icons.comment,
                        size: 16,
                        color: Colors.grey,
                      ),
                      Text(
                        " ${comment.count}",
                        style: const TextStyle(
                          color: Colors.grey,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ],
          ),
          Container(
            height: 5,
          ),
          Text(
            comment.comment,
            style: const TextStyle(
              fontSize: 14,
            ),
          ),
        ],
      ),
    );
  }
}
