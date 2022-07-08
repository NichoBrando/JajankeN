import { App, aws_ec2, Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import dotenv from 'dotenv';

dotenv.config();

export class MyStack extends Stack {
  constructor(scope: Construct, id: string, props: StackProps = {}) {
    super(scope, id, props);

    const vpc = new aws_ec2.Vpc(this, 'JajankenVPC');

    new aws_ec2.Instance(this, 'JajankenEC2', {
        vpc,
        instanceType: aws_ec2.InstanceType.of(aws_ec2.InstanceClass.T2, aws_ec2.InstanceSize.MICRO),
        machineImage: new aws_ec2.AmazonLinuxImage(),
        vpcSubnets: {
            subnetType: aws_ec2.SubnetType.PUBLIC,
        },
        keyName: 'jajanken',
        securityGroup: new aws_ec2.SecurityGroup(this, 'JajankenSG', {
            vpc,
            allowAllOutbound: true,
        }),
    });
  }
}

const devEnv = {
  account: process.env.CDK_DEFAULT_ACCOUNT,
  region: process.env.CDK_DEFAULT_REGION,
};

const app = new App();

new MyStack(app, 'jajanken-dev', { env: devEnv });

app.synth();