<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>8_Add FOP - CreditCard</name>
   <tag></tag>
   <elementGuidId>4c68a9b1-bb59-43ce-b4de-b015d8a3be55</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;FormOfPaymentPaymentCard\&quot; : {\n    \&quot;id\&quot; : \&quot;formOfPayment_4\&quot;,\n    \&quot;FormOfPaymentRef\&quot; : \&quot;formOfPayment_4\&quot;,\n    \&quot;PaymentCard\&quot; : {\n      \&quot;@type\&quot; : \&quot;PaymentCard\&quot;,\n      \&quot;id\&quot; : \&quot;paymentCard_1\&quot;,\n      \&quot;expireDate\&quot; : \&quot;0624\&quot;,\n      \&quot;CardType\&quot; : \&quot;Credit\&quot;,\n      \&quot;CardCode\&quot; : \&quot;VI\&quot;,\n      \&quot;CardHolderName\&quot; : \&quot;MUGI\&quot;,\n      \&quot;CardNumber\&quot; : {\n        \&quot;PlainText\&quot; : \&quot;4005520000000129\&quot;\n      }\n    }\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ace3e032-f330-4a41-8134-e6b4b4ed71eb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5b532a3e-6f62-4642-b998-df2026f4c6c9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>5f50dbb8-4027-45dc-ab60-f9ff34223e85</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>ac5ad8b2-7c1e-42d5-9379-bb84f1289131</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>29200199-8187-4074-a73e-49dd73bf52e8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>ad781ade-391c-4f7c-85f7-345e13100fea</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>cf383c78-ec6c-47c0-ab78-22a6c6d86f37</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>d3f4bc12-6fe0-433e-8cc3-e0f0ded34569</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sAddFOPURL}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.eCacheId</defaultValue>
      <description></description>
      <id>638fb1ae-fa82-4c11-bcb3-d63cca3a7dc7</id>
      <masked>false</masked>
      <name>eCacheId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAccessGroup</defaultValue>
      <description></description>
      <id>a0f7ff78-1adb-47ee-aded-45c8b3a067d4</id>
      <masked>false</masked>
      <name>sAccessGroup</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAddFOPURL</defaultValue>
      <description></description>
      <id>1b69895e-614c-4ae4-8f44-31a5cdcd6497</id>
      <masked>false</masked>
      <name>sAddFOPURL</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
