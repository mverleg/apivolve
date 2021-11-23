
# Run as follows to create a evolution merge conflict:
#

FROM mverleg/ubuntu_dev_tools:2021-11-21_2

RUN git init &&\
    git checkout -b initial

ADD base.apiv apivolve/v0.0.0.apiv




