# HASS Proxy

This is a simple proxy server that allows you to access data of your Home Assistant 
instance from outside your network. 

## Home Assistant configuration


## Development helpers

### Docker test database 🐋

- Pull newest postgres image 
    ```` shell
    docker pull postgres
    ````

- Create test database
    ```` shell
    docker run --name hass_proxy-test-db -e POSTGRES_PASSWORD=mysecretpassword -e POSTGRES_DB=hass_proxy -p 5432:5432 -d postgres
    ````
  
- Add the new database url to the .env file:```postgresql://postgres:mysecretpassword@localhost:5432/mytestdb```



### sqlx migration shortcut
First install the sqlx-cli:
```` shell
cargo install sqlx-cli --no-default-features --features native-tls,postgres
````


To rerun changed migrations, execute the following command:
```` shell
sqlx database drop -y; sqlx database create; sqlx migrate run
````

## 🏛️ License
```
HASS-Proxy · A simple proxy server that allows you to access data of 
your Home Assistant instance from outside your network.
Copyright (C) 2024  Yannick W.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```