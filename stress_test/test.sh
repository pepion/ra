# seq 1 500 | xargs -n1 -P500 bash -c 'i=$0; j=${i}; url="http://127.0.0.1:8080/joke"; curl -s $url -o "./jokes/joke_${i}.txt"'

seq 1 500 | xargs -n1 -P500 bash -c 'i=$0; j=${i}; url="http://127.0.0.1:8080/joke/n${i}"; curl -s $url'
