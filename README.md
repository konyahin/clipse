# Clipse
Gopher browser.

## Test gopher page
You can easily test the content of a gopher page using the following command. 
Simply specify the path in the first string literal (make sure to keep the `\r\n`), and the host in the second.
```sh
printf "/\r\n" | nc "sdf.org" 70
```