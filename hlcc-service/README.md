# hlcc-service

Hlcc-service is an asychronous web api for parsing hormone level strings and returning the computed result. Such a string might be „Testosterone 1.8nmol/l to ng/dl“.

The servie can be queried by sending a GET request with the expression encoded as query parameter. Example `https://api.hlcc.haj.gf/?q=t%201.8nmol/l%20to%20ng/dl`.
