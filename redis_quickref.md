redis_quickref.md


##### Redis Lists Reference

new list:
redis 127.0.0.1:6379> rpush players jason rick mary marcus noopy emily emily_mom
(integer) 7

push to right side of list:
redis 127.0.0.1:6379> rpush players "kanye"
(integer) 8

shift/push to left side of list (at index 0):
redis 127.0.0.1:6379> lpush players "kim k"
(integer) 9

pop from left (first item in list):
redis 127.0.0.1:6379> lpop players
"kim k"

pop from right (last item in list):
redis 127.0.0.1:6379> rpop players
"kanye"

list all items (here 0 is the beginning of the list and -1 is the last):
redis 127.0.0.1:6379> lrange players 0 -1
1) "jason"
2) "rick"
3) "mary"
4) "marcus"
5) "noopy"
6) "emily"
7) "emily_mom"


delete list:
redis 127.0.0.1:6379> del players
(integer) 1
