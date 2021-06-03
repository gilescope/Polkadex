export ANSIBLE_CONFIG=ansible.cfg
rm -rf /tmp/aws_inventory # Do this when ever you are unable to find hosts
ansible-playbook provision-ec2-mumbai.yml
ansible-playbook deploy-nodes.yml