FROM gitpod/workspace-full-vnc

USER root

RUN apt-get update && apt-get install nim