#include<iostream>
using namespace std;
int maxOf2(int num1,int num2){
    return (num1>=num2?num1:num2);
}
int maxOf3(int num1,int num2,int num3){
    if(num1>=num2){
        if(num1>=num3){
            return num1;
        }
        else{
            return num3;
        }
    }
    else if(num2>=num3){
        return num2;
    }
    else{
        return num3;
    }
    
}
int maxOf4(int num1,int num2,int num3,int num4){
    if(num1>=num2){
        if(num1>=num3){
            if(num1>=num4){
                return num1;
            }
            else{
                return num4;
            }
        }
        else{
            if(num3>=num4){
                return num3;
            }
            else{
                return num4;
            }
        }
    }
    else if(num2>=num3){
        if(num2>=num4){
            return num2;
        }
        else{
            return num4;
        }
    }
    else if(num3>=num4){
        return num3;
    }
    else{
        return num4;
    }
}
int main(){
    int n1,n2,n3,n4,ch;
    cout<<"\n1. Max of 2 Numbers\n2. Max of 3 Numbers\n3. Max of 4 Numbers";
    cout<<"Enter Choice : ";
    cin>>ch;
    switch (ch)
    {
    case 1:
        cout<<"\nEnter 2 Numbers : ";
        cin>>n1>>n2;
        cout<<"Max of 2 : "<<maxOf2(n1,n2);
        break;
    case 2:
        cout<<"\nEnter 3 Numbers : ";
        cin>>n1>>n2>>n3;
        cout<<"\nMax of 3 : "<<maxOf3(n1,n2,n3);
        break;
    case 3: 
        cout<<"\nEnter 4 Numbers :  ";
        cin>>n1>>n2>>n3>>n4;
        cout<<"\nMax of 4 : "<<maxOf4(n1,n2,n3,n4);
    default:
        cout<<"\nInvalid Input";
        break;
    }
    return 0;
    
}