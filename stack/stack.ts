import { App } from '@aws-cdk/core';
import { LambdaStack } from './lib/lambdaStack';

export default class Stack {
    public lambdaStack: LambdaStack;

    constructor(app: App) {
        this.lambdaStack = new LambdaStack(app, 'ytakada-dev-stack', {})
    }
}

const app = new App;
new Stack(app);
