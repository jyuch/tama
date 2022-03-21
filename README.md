tama
====
Console tool for Apache Tomcat manager.

## About
This tool provides below functions via Tomcat manager.

- List currently deployed applications
- Deploy a new application
- Undeploy an existing application
- Start an existing application
- Stop an existing application
- Reload an existing application

## Getting started

### Configuration
This tool retrieves Tomcat information from environment variables.

|Environment variables|Description|Example|
|:-|:-|:-|
|`TOMCAT_HOST`|Tomcat host URL|`http://localhost:8080`|
|`TOMCAT_USER`|Tomcat script manager role|`manager`|
|`TOMCAT_PASSWORD`|Tomcat script manager role password|`p@ssw0rd`|

### List currently deployed applications

```shell
tama list
```

```
context path         | status  | session | directory            | version   
/                    | running |       0 | ROOT                 | N/A       
/examples            | running |       0 | examples             | N/A       
/host-manager        | running |       0 | host-manager         | N/A       
/manager             | running |       0 | manager              | N/A       
/docs                | running |       0 | docs                 | N/A       
```

### Deploy a new application

Deploy `foo.war` to context path `/foo` without version.

```shell
tama deploy --context-path /foo --war-file "/path/to/war/foo.war"
```

Deploy `foo.war` to context path `/foo` with version.

```shell
tama deploy --context-path /foo --war-file "/path/to/war/foo.war" --parallel
```

### Undeploy an existing application

Undeploy context path `/foo`.

```shell
tama undeploy --context-path /foo
```

Undeploy context path `/foo` with version.

```shell
tama undeploy --context-path /foo##00001
```

### Start an existing application

Start context path `/foo`.

```shell
tama start --context-path /foo
```

### Stop an existing application

Stop context path `/foo`.

```shell
tama stop --context-path /foo
```

### Reload an existing application

Reload context path `/foo`.

```shell
tama reload --context-path /foo
```
