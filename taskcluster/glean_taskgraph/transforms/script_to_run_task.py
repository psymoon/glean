# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from __future__ import absolute_import, print_function, unicode_literals
from taskgraph.transforms.base import TransformSequence

transforms = TransformSequence()


def script_to_bash_command(script):
    return [
        "/bin/bash",
        "--login",
        "-c",
        "cat <<'SCRIPT' > ../script.sh && bash -e ../script.sh\n{}\nSCRIPT".format(script)
    ]


@transforms.add
def build_task(config, tasks):
    for task in tasks:
        print(task)
        script = task["run"].pop("script")
        bash_command = script_to_bash_command(script)
        task["run"]["command"] = bash_command

        yield task
