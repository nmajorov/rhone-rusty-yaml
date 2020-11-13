New proposal for the build:


```yaml
    ---
    environment:
      KEYKLOAK_URL: "http://localhost:8080"
      DATABASE_ADDR: "test-test"
      SECRET_URL: "hfsrtasdr"

    pre_build:
      - ./scripts/build.sh
      -
      - "pip install ansible"
      - ""
      - ""
        pre_build_ansible:
          - ./ansible/playbook.yaml

        post_build_ansible:
          - ./ansible/


      - task: "check python version"
      	command: ["/bin/bash"]
        args:
          - "python version"
          - "rm dir"
      - task: "test"
      	in_container: false
        command: ["/bin/bash"]
        args: ['-c', 'mkdir -p /workspace/tar-scratch-space/ && tar -xvf /workspace/customworkspace/rules_docker-master.tar -C /workspace/tar-scratch-space/']

      - task: "test"
      	command: ["/bin/bash"]
        args: ['-c',"python version && pip list"]


    build:
      - task: "test"
      	script: |
        #!/usr/bin/env bash
        /bin/my-binary
        test.sh


    post_build:
      - task: "post_build"
      	script: "$(workspace)/podman/run_podman.sh"

```

same equivalent in json:

```javascript
{
  "environment": {
    "KEYKLOAK_URL": "http://localhost:8080",
    "DATABASE_ADDR": "test-test",
    "SECRET_URL": "hfsrtasdr"
  },
  "pre_build": [
    {
      "task": "check python version",
      "in_container": false,
      "command": [
        "/bin/bash"
      ],
      "args": [
        "python version",
        "rm dir"
      ]
    },
    {
      "task": "test",
      "in_container": false,
      "command": [
        "/bin/bash"
      ],
      "args": [
        "-c",
        "mkdir -p /workspace/tar-scratch-space/ && tar -xvf /workspace/customworkspace/rules_docker-master.tar -C /workspace/tar-scratch-space/"
      ]
    },
    {
      "task": "test",
      "command": [
        "/bin/bash"
      ],
      "args": [
        "-c",
        "python version && pip list"
      ]
    }
  ],
  "build": [
    {
      "task": "test",
      "script": "#!/usr/bin/env bash\n/bin/my-binary\ntest.sh\n"
    }
  ],
  "post_build": [
    {
      "task": "post_build",
      "script": "$(workspace)/podman/run_podman.sh"
    }
  ]
}

```


same equivalent in json:

```javascript
{
  "environment": {
    "KEYKLOAK_URL": "http://localhost:8080",
    "DATABASE_ADDR": "test-test",
    "SECRET_URL": "hfsrtasdr"
  },
  "pre_build": [
    {
      "task": "check python version",
      "in_container": false,
      "command": [
        "/bin/bash"
      ],
      "args": [
        "python version",
        "rm dir"
      ]
    },
    {
      "task": "test",
      "in_container": false,
      "command": [
        "/bin/bash"
      ],
      "args": [
        "-c",
        "mkdir -p /workspace/tar-scratch-space/ && tar -xvf /workspace/customworkspace/rules_docker-master.tar -C /workspace/tar-scratch-space/"
      ]
    },
    {
      "task": "test",
      "command": [
        "/bin/bash"
      ],
      "args": [
        "-c",
        "python version && pip list"
      ]
    }
  ],
  "build": [
    {
      "task": "test",
      "script": "#!/usr/bin/env bash\n/bin/my-binary\ntest.sh\n"
    }
  ],
  "post_build": [
    {
      "task": "post_build",
      "script": "$(workspace)/podman/run_podman.sh"
    }
  ]
}

```
