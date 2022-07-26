package methods
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.codehaus.groovy.scriptom.ActiveXObject;
import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords
import com.kms.katalon.core.annotation.BeforeTestCase
import com.kms.katalon.core.annotation.BeforeTestSuite
import com.kms.katalon.core.annotation.AfterTestCase
import com.kms.katalon.core.annotation.AfterTestSuite
import com.kms.katalon.core.context.TestCaseContext
import com.kms.katalon.core.context.TestSuiteContext
import internal.GlobalVariable
import static groovy.io.FileType.FILES
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import groovy.io.FileType
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import org.codehaus.groovy.scriptom.ActiveXObject
import internal.GlobalVariable as GlobalVariable
import jxl.*
import jxl.write.*
import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException
import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.webservice.verification.WSResponseManager




import org.apache.poi.xssf.usermodel.XSSFSheet;
import org.apache.poi.xssf.usermodel.XSSFWorkbook;
import org.apache.poi.xssf.*;
import org.apache.poi.hssf.util.HSSFColor
import org.apache.poi.ss.usermodel.*;
import pnrcreation.*;

class APIKeywords {
	/**
	 * Send request and verify status code
	 * @param request request object, must be an instance of RequestObject
	 * @param expectedStatusCode
	 * @return a boolean to indicate whether the response status code equals the expected one
	 */
	@Keyword
	def verifyStatusCode(TestObject request, int expectedStatusCode) {
		if (request instanceof RequestObject) {
			RequestObject requestObject = (RequestObject) request
			ResponseObject response = WSBuiltInKeywords.sendRequest(requestObject)
			if (response.getStatusCode() == expectedStatusCode) {
				KeywordUtil.markPassed("Response status codes match")
			} else {
				KeywordUtil.markFailed("Response status code not match. Expected: " +
						expectedStatusCode + " - Actual: " + response.getStatusCode() )
			}
		} else {
			KeywordUtil.markFailed(request.getObjectId() + " is not a RequestObject")
		}
	}

	/**
	 * Add Header basic authorization field,
	 * this field value is Base64 encoded token from user name and password
	 * @param request object, must be an instance of RequestObject
	 * @param username username
	 * @param password password
	 * @return the original request object with basic authorization header field added
	 */
	@Keyword
	def addBasicAuthorizationProperty(TestObject request, String username, String password) {
		if (request instanceof RequestObject) {
			String authorizationValue = username + ":" + password
			authorizationValue = "Basic " + authorizationValue.bytes.encodeBase64().toString()

			// Find available basic authorization field and change its value to the new one, if any
			List<TestObjectProperty> headerProperties = request.getHttpHeaderProperties()

			boolean fieldExist = false
			for (int i = 0; i < headerProperties.size(); i++) {
				TestObjectProperty headerField = headerProperties.get(i)
				if (headerField.getName().equals('Authorization')) {
					KeywordUtil.logInfo("Found existent basic authorization field. Replacing its value.")
					headerField.setValue(authorizationValue)
					fieldExist = true
					break
				}
			}

			if (!fieldExist) {
				TestObjectProperty authorizationProperty = new TestObjectProperty("Authorization",
						ConditionType.EQUALS, authorizationValue, true)
				headerProperties.add(authorizationProperty)
			}
			KeywordUtil.markPassed("Basic authorization field has been added to request header")
		} else {
			KeywordUtil.markFailed(request.getObjectId() + "is not a RequestObject")
		}
		return request
	}



	@Keyword
	def SearchCatalogueIDAndProductID(def SearchResponse) {



		ErrorMessage (SearchResponse)

		def slurper = new groovy.json.JsonSlurper()

		def sSearchResponse = slurper.parseText(SearchResponse.getResponseBodyContent())

		def SearchToken = sSearchResponse.CatalogOfferingsResponse.CatalogOfferings.Identifier.value

		GlobalVariable.sSearchToken = SearchToken

		println('Value of Search Identifier Token is ' + GlobalVariable.sSearchToken)



		//def SearchToken = sSearchResponse.CatalogOfferingsResponse.CatalogOfferings.Identifier.value

		//GlobalVariable.sSearchToken = SearchToken

		//println('Value of Search Identifier Token is ' + GlobalVariable.sSearchToken)

		//WS.verifyResponseStatusCode(SearchResponse, 200)

		def CatalogOffering = sSearchResponse.CatalogOfferingsResponse.CatalogOfferings.CatalogOffering

		List<String> CatalogOfferingList = new ArrayList<>()
		List<String> ProductList = new ArrayList<>()
		def TotalProduct = ''
		def TotalCatalogOffering = ''
		//String[] CatalogOfferingList = [] as String[]
		//def CatalogOfferingList = []


		for(def i = 0; i < CatalogOffering.size(); i++) {

			def CatalogOfferingID = CatalogOffering[i]


			CatalogOfferingList.add(CatalogOffering[i].id)


			def ProductOptions = CatalogOffering[i].ProductOptions

			for(def j = 0; j < ProductOptions.size(); j++) {

				//println('ProductOptions size is '+ ProductOptions.size())

				def Product = ProductOptions[j].Product

				for(def k = 0; k < Product.size(); k++) {

					//println('For Catalogue id '+ CatalogOffering[i].id +' The Product id is '+ CatalogOfferingID.ProductOptions[j].Product[k].id)
					ProductList.add(CatalogOfferingID.ProductOptions[j].Product[k].id)
					TotalProduct = TotalProduct +','+ CatalogOfferingID.ProductOptions[j].Product[k].id


				}



			}

			//println('The total product id are' + TotalProduct)
			TotalCatalogOffering = TotalCatalogOffering +','+ CatalogOffering[i].id
			println('For CatalogOffering id' + TotalCatalogOffering + ' The Total Product IDs are' + TotalProduct)

			TotalCatalogOffering = ''
			TotalProduct = ''

		}







		for(def i = 0; i < CatalogOffering.size(); i++) {

			def CatalogOfferingID = CatalogOffering[i]

			//CatalogOfferingList.add(CatalogOffering[i].id)


			def ProductOptions = CatalogOffering[i].ProductOptions

			for(def j = 0; j < ProductOptions.size(); j++) {

				//println('ProductOptions size is '+ ProductOptions.size())

				def Product = ProductOptions[j].Product

				for(def k = 0; k < Product.size(); k++) {

					//println('For Catalogue id '+ CatalogOffering[i].id +' The Product id is '+ CatalogOfferingID.ProductOptions[j].Product[k].id)
					//ProductList.add(CatalogOfferingID.ProductOptions[j].Product[k].id)
					//TotalProduct = TotalProduct +','+ CatalogOfferingID.ProductOptions[j].Product[k].id

					if(i == 0 && j == 0 && k == 1) {

						def CatalogOfferingIdentifierID = CatalogOffering[i].id
						def ProductFromID = CatalogOfferingID.ProductOptions[j].Product[k].id

						//break;
						println('ProductFromID is '+ ProductFromID)
						println('CatalogOfferingIdentifier is '+ CatalogOfferingIdentifierID)
						GlobalVariable.CatalogOfferingIdentifierID = CatalogOfferingIdentifierID
						GlobalVariable.ProductFromID = ProductFromID


					}

					else if (i == 0 && j == 1 && k == 1) {

						def CatalogOfferingIdentifierID = CatalogOffering[i].id
						def ProductToID = CatalogOfferingID.ProductOptions[j].Product[k].id
						println('ProductToID is '+ ProductToID)
						println('CatalogOfferingIdentifier is '+ CatalogOfferingIdentifierID)
						GlobalVariable.CatalogOfferingIdentifierID = CatalogOfferingIdentifierID
						GlobalVariable.ProductToID = ProductToID
						//break;


					}

					else if (i == 0 && j == 2 && k == 0) {

						def CatalogOfferingIdentifierID = CatalogOffering[i].id
						def ProductCircleID = CatalogOfferingID.ProductOptions[j].Product[k].id
						println('ProductCircleID is '+ ProductCircleID)
						println('CatalogOfferingIdentifier is '+ CatalogOfferingIdentifierID)
						GlobalVariable.CatalogOfferingIdentifierID = CatalogOfferingIdentifierID
						GlobalVariable.ProductCircleID = ProductCircleID
						//break;


					}



				}



			}



			//println('The total product id are' + TotalProduct)
			//TotalCatalogOffering = TotalCatalogOffering +','+ CatalogOffering[i].id
			//println('For CatalogOffering id' + TotalCatalogOffering + ' The Total Product IDs are' + TotalProduct)

			//TotalCatalogOffering = ''
			//TotalProduct = ''

		}





		//x.CatalogOfferingsResponse.CatalogOfferings.CatalogOffering[0].ProductOptions[0].Product[0].id
		//x.CatalogOfferingsResponse.CatalogOfferings.CatalogOffering[0].ProductOptions[1].Product[0].id












		//def hello = CatalogOfferingList.size()

		//println('Total CatalogofferingIDs are '+ CatalogOfferingList.size())
		//CatalogOfferingList.each { println it }

		//for (int i = 0; i < CatalogOfferingList.size();i++)

		// {

		//	   println(CatalogOfferingList.get(i));

		//}




	}


	@Keyword
	def SearchToken(def SearchResponse) {

		ErrorMessage (SearchResponse)

		def slurper = new groovy.json.JsonSlurper()

		def sSearchResponse = slurper.parseText(SearchResponse.getResponseBodyContent())

		def SearchToken = sSearchResponse.CatalogOfferingsResponse.CatalogOfferings.Identifier.value

		GlobalVariable.sSearchToken = SearchToken

		println('Value of Search Identifier Token is ' + GlobalVariable.sSearchToken)


	}



	@Keyword
	def InitiateWorkBench(def InitiateWorkBenchResponse) {

		ErrorMessage (InitiateWorkBenchResponse)

		def slurper = new groovy.json.JsonSlurper()

		def sInitiateWorkBench = slurper.parseText(InitiateWorkBenchResponse.getResponseBodyContent())

		def WorkBench = sInitiateWorkBench.ReservationResponse.Reservation.Identifier.value

		GlobalVariable.reservationId = WorkBench

		GlobalVariable.catalogOfferingsIdentifier = GlobalVariable.sSearchToken

		println('Value of WorkBench catalogOfferingsIdentifier ' + GlobalVariable.catalogOfferingsIdentifier)

		println('Value of Reservation ' + GlobalVariable.reservationId)

	}

	@Keyword
	def Commit(def Commit) {

		ErrorMessage (Commit)

		def slurper = new groovy.json.JsonSlurper()

		def sCommit = slurper.parseText(Commit.getResponseBodyContent())

		def sRecordLocator = sCommit.ReservationResponse.Reservation.Receipt[0].Confirmation.Locator.value
		//x.ReservationResponse.Reservation.Receipt[0].Confirmation.Locator.value
		//x.ReservationResponse.Reservation.Receipt[0].Confirmation.Locator.value
		GlobalVariable.PNR = sRecordLocator

		if (sRecordLocator.length() == 6) {
			//println('The length of the PNR is ' + sRecordLocator.length())
			//WS.comment('The Galileo Record Locator is ' + GlobalVariable.PNR )

			//println('The Galileo Record Locator is '+GlobalVariable.PNR)
			KeywordUtil.logInfo('The Galileo Record Locator is '+GlobalVariable.PNR)
			//println('The length of the PNR is ' + sRecordLocator.length())

		} else {
			//println('The Galileo Record Locator is not generated ')
			KeywordUtil.markFailedAndStop('The Galileo Record Locator is not generated ')
		}

	}

	@Keyword
	def TicketNumber(def TicketIssuance) {

		ErrorMessage(TicketIssuance)

		def slurper = new groovy.json.JsonSlurper()

		def sTicketIssuance = slurper.parseText(TicketIssuance.getResponseBodyContent())

		//def sTicketNumber = sTicketIssuance.ReservationResponse.Reservation.Receipt[3].Document[0].Number
		//	x.ReservationResponse.Reservation.Receipt[3].Document[5].Number
		//	x.ReservationResponse.Reservation.Receipt[3].Document[1].Number
		//	x.ReservationResponse.Reservation.Receipt[3].Document[0].Number



		def sTicketSize = sTicketIssuance.ReservationResponse.Reservation.Receipt

		List<String> TicketNumber = new ArrayList<>()

		for(def i = 0; i < sTicketSize.size(); i++) {

			def sType = sTicketSize[i].find().toString()

			if (sType == '@type=ReceiptPayment') {

				def sDocumentSize = sTicketIssuance.ReservationResponse.Reservation.Receipt[i].Document

				//println('The document size is ' + sDocumentSize)

				for (def j = 0; j < sDocumentSize.size(); j++) {

					TicketNumber.add(sTicketSize[i].Document[j].Number)

				}

			}

		}




		for (int j = 0; j < TicketNumber.size();j++)

		{
			def sTicketNumber = TicketNumber.get(j)

			if (sTicketNumber.length() == 13)

			{

				def k = j + 1
				GlobalVariable.sOldTicket = sTicketNumber;
				//println('The ' +j+ 'Ticket no is ' + GlobalVariable.sOldTicket)
				KeywordUtil.logInfo('The ' +k+ ' Ticket no is ' + GlobalVariable.sOldTicket)


			}



		}



		String iTicketNumbersize =  TicketNumber.size()


		if (iTicketNumbersize == GlobalVariable.sPassengerCount)
		{
			KeywordUtil.logInfo('All Etkt issued')
		}
		else
		{
			KeywordUtil.markFailedAndStop('Either All or One of the E-ticket Not issued, Check the Response Log')

		}



	}



	@Keyword
	def ReservationCommit(def ReservationCommit) {

		ErrorMessage(ReservationCommit)

		def slurper = new groovy.json.JsonSlurper()

		def sReservationCommit = slurper.parseText(ReservationCommit.getResponseBodyContent())

		def sTicketSize = sReservationCommit.ReservationResponse.Reservation.Receipt

		KeywordUtil.logInfo('The Galileo Record Locator is '+GlobalVariable.PNR)

		List<String> OpenTicketNumber = new ArrayList<>()

		List<String> ExchTicketNumber = new ArrayList<>()

		for(def i = 0; i < sTicketSize.size(); i++) {

			def sType = sTicketSize[i].find().toString()

			//println('hello'+sType)


			//if (sType.contains('ReceiptPayment') {
			if (sType == '@type=ReceiptPayment') {

				//def sOldTicket = sTicketSize[i].Document[0].Number
				//def sNewTicket = sTicketSize[i].Document[0].Number
				//ReservationResponse.Reservation.Receipt[3].Document[0].Number
				//ReservationResponse.Reservation.Receipt[4].Document[0].Number
				def sDocumentSize = sReservationCommit.ReservationResponse.Reservation.Receipt[i].Document

				//println('The document size is ' + sDocumentSize)

				for (def j = 0; j < sDocumentSize.size(); j++) {


					def sDocumentType = sDocumentSize[j].find().toString()

					if (sDocumentType == '@type=DocumentTicketExchange') {


						ExchTicketNumber.add(sTicketSize[i].Document[j].Number)

						//GlobalVariable.sOldTicket = sTicketSize[i].Document[j].Number

						//KeywordUtil.logInfo('The Exchanged Ticket is ' + GlobalVariable.sOldTicket)



					}

					else if(sDocumentType == '@type=DocumentTicket'){


						OpenTicketNumber.add(sTicketSize[i].Document[j].Number)

						//GlobalVariable.sNewTicket = sTicketSize[i].Document[j].Number;
						//KeywordUtil.logInfo('The Open Ticket is ' + GlobalVariable.sNewTicket)


					}


				}


			}

		}

		//println(ExchTicketNumber.size())
		//println(ExchTicketNumber.get(0))


		def Message

		if (ExchTicketNumber.size() == 0) {

			KeywordUtil.markFailedAndStop('The Eticket could not be reissued, kindly check the response log')

		}

		else {





			for (int j = 0; j < OpenTicketNumber.size();j++)

			{

				int k = j + 1

				String sOpenTicketNumber = OpenTicketNumber.get(j);
				String sExchTicketNumber = ExchTicketNumber.get(j);

				if (sOpenTicketNumber.length() == 13 && sExchTicketNumber.length() == 13) {

					GlobalVariable.sOldTicket = ExchTicketNumber.get(j);

					KeywordUtil.logInfo('The Exchanged Ticket for '+k+' Passenger is ' + GlobalVariable.sOldTicket)

					GlobalVariable.sNewTicket = OpenTicketNumber.get(j);


					KeywordUtil.markPassed('Ticket issued for exchanged offer for '+k+' Passenger ' +  GlobalVariable.sNewTicket)

					//def Message = 'This PNR is exchanged and new ticket issuedexchanged ticket no is ' + GlobalVariable.sNewTicket
					Message = 'This PNR is rebooked and exchanged ,the new ticket issued'

				}

				else {

					KeywordUtil.markFailed('Ticket not issued for exchanged offer for '+k+' Passenger , kindly check the response log')

				}

			}

		}



		if (GlobalVariable.sExecutionFlow == 'PNRGenerator')

		{
			UpdateExcel(Message)

		}


	}



	@Keyword
	def WorkBenchWithPNR(def WorkBenchWithPNR) {


		ErrorMessage(WorkBenchWithPNR)
		def slurper = new groovy.json.JsonSlurper()

		def sWorkBenchWithPNR = slurper.parseText(WorkBenchWithPNR.getResponseBodyContent())

		def sWorkBenchIdentifierWithPNR = sWorkBenchWithPNR.ReservationResponse.Identifier.value


		GlobalVariable.eCacheId = sWorkBenchIdentifierWithPNR

		println('The WorkBenchIdentifierWithPNR is ' + GlobalVariable.eCacheId)

	}


	@Keyword
	def ErrorMessage(def ErrorMessage) {


		def slurper = new groovy.json.JsonSlurper()

		//def sErrorMessage = slurper.parseText('{"CatalogOfferingsResponse":{"transactionId":"Samurai_Test","Result":{"Error":[{"@type":"ErrorDetail","StatusCode":521,"Message":"No Offers Found for channel  c4069c1e-4820-4063-9e4d-5be7d9c40291","SourceCode":"9015"}]}}}')
		//x.CatalogOfferingsAirChangeResponse.Result.Error[0].Message
		def sErrorMessage = slurper.parseText(ErrorMessage.getResponseBodyContent())




		def sResponse = sErrorMessage.findAll()

		sResponse.each {

			def sKeys =  " $it.key "

			sKeys = sKeys.toString().trim()

			//println(sKeys)

			if (sKeys == 'TicketChangeEligibilityListResponse') {


				def EligibilityListResponse = sErrorMessage.TicketChangeEligibilityListResponse.findAll()

				EligibilityListResponse.each {

					def sKey =  " $it.key "

					sKey = sKey.toString().trim()

					//println(sKey)

					if (sKey == 'Result') {

						def sMessage = 	sErrorMessage.TicketChangeEligibilityListResponse.Result.Error[0].Message
						//println(sMessage)

						//KeywordUtil.markFailedAndStop(sMessage)
						KeywordUtil.markFailed(sMessage)
						UpdateExcel(sMessage)
						//KeywordUtil.logInfo(sMessage)



					}

				}

			}

			else if (sKeys == 'CatalogOfferingsResponse') {


				def properties = sErrorMessage.CatalogOfferingsResponse.findAll()

				//properties.each { println "Hex Code: $it.key = Color Name: $it.value" }

				properties.each {

					def sKey =  " $it.key "

					sKey = sKey.toString().trim()

					//println(sKey)

					if (sKey == 'Result') {

						def sError = sErrorMessage.CatalogOfferingsResponse.Result.Error[0].Message

						//x.CatalogOfferingsResponse.Result.Error[0].Message
						//println(sError)
						//KeywordUtil.markFailed(sError)
						KeywordUtil.markFailedAndStop(sError)
						//KeywordUtil.logInfo(sError)


					}


				}


			}

			else if (sKeys == 'CatalogOfferingsAirChangeResponse') {

				def properties = sErrorMessage.CatalogOfferingsAirChangeResponse.findAll()

				//properties.each { println "Hex Code: $it.key = Color Name: $it.value" }

				properties.each {

					def sKey =  " $it.key "

					sKey = sKey.toString().trim()

					//println(sKey)

					if (sKey == 'Result') {

						def sError = sErrorMessage.CatalogOfferingsAirChangeResponse.Result.Error[0].Message

						//x.CatalogOfferingsResponse.Result.Error[0].Message
						//println(sError)
						//KeywordUtil.markFailed(sError)
						KeywordUtil.markFailedAndStop(sError)
						//KeywordUtil.logInfo(sError)


					}


				}



			}


			else if (sKeys == 'TravelerResponse') {


				def properties = sErrorMessage.TravelerResponse.findAll()

				//properties.each { println "Hex Code: $it.key = Color Name: $it.value" }

				properties.each {

					def sKey =  " $it.key "

					sKey = sKey.toString().trim()

					//println(sKey)

					if (sKey == 'Result') {

						def sError = sErrorMessage.TravelerResponse.Result.Error[0].Message

						//TravelerResponse.Result.Error[0].Message
						//TravelerResponse.Result.Error
						KeywordUtil.markFailedAndStop(sError)
						//KeywordUtil.logInfo(sError)


					}


				}



			}








		}




	}







	@Keyword
	def ErrorMessagePostEligibility(def ErrorMessage) {


		def slurper = new groovy.json.JsonSlurper()

		//def sErrorMessage = slurper.parseText('{"CatalogOfferingsResponse":{"transactionId":"Samurai_Test","Result":{"Error":[{"@type":"ErrorDetail","StatusCode":521,"Message":"No Offers Found for channel  c4069c1e-4820-4063-9e4d-5be7d9c40291","SourceCode":"9015"}]}}}')
		//x.CatalogOfferingsAirChangeResponse.Result.Error[0].Message



		def sErrorMessage = slurper.parseText(ErrorMessage.getResponseBodyContent())

		def properties = sErrorMessage.CatalogOfferingsAirChangeResponse.findAll()

		//properties.each { println "Hex Code: $it.key = Color Name: $it.value" }

		properties.each {

			def sKey =  " $it.key "

			sKey = sKey.toString().trim()

			//println(sKey)

			if (sKey == 'Result') {

				def sError = sErrorMessage.CatalogOfferingsAirChangeResponse.Result.Error[0].Message

				//x.CatalogOfferingsResponse.Result.Error[0].Message
				//println(sError)
				//KeywordUtil.markFailed(sError)
				KeywordUtil.markFailedAndStop(sError)
				KeywordUtil.logInfo(sError)


			}


		}


	}

	@Keyword
	def AddTravellerErrorMessage(def ErrorMessage) {


		def slurper = new groovy.json.JsonSlurper()

		//def sErrorMessage = slurper.parseText('{"CatalogOfferingsResponse":{"transactionId":"Samurai_Test","Result":{"Error":[{"@type":"ErrorDetail","StatusCode":521,"Message":"No Offers Found for channel  c4069c1e-4820-4063-9e4d-5be7d9c40291","SourceCode":"9015"}]}}}')

		def sErrorMessage = slurper.parseText(ErrorMessage.getResponseBodyContent())

		def properties = sErrorMessage.TravelerResponse.findAll()

		//properties.each { println "Hex Code: $it.key = Color Name: $it.value" }

		properties.each {

			def sKey =  " $it.key "

			sKey = sKey.toString().trim()

			//println(sKey)

			if (sKey == 'Result') {

				def sError = sErrorMessage.TravelerResponse.Result.Error[0].Message

				//TravelerResponse.Result.Error[0].Message
				//TravelerResponse.Result.Error
				KeywordUtil.markFailedAndStop(sError)
				//KeywordUtil.logInfo(sError)


			}


		}


	}


	@Keyword
	def FareValidation(def PriceResponse) {

		ErrorMessage(PriceResponse)

		def slurper = new groovy.json.JsonSlurper()
		def sPriceResponse = slurper.parseText(PriceResponse.getResponseBodyContent())


		def sBasePrice = sPriceResponse.OfferListResponse.OfferID[0].Price.Base
		def sTotalTaxes = sPriceResponse.OfferListResponse.OfferID[0].Price.TotalTaxes
		def sTotalFees = sPriceResponse.OfferListResponse.OfferID[0].Price.TotalFees
		def sTotalPrice = sPriceResponse.OfferListResponse.OfferID[0].Price.TotalPrice

		def sBaseTaxFee = sBasePrice + sTotalTaxes + sTotalFees

		if(sTotalPrice == sBaseTaxFee )

		{
			KeywordUtil.markPassed('The Total Price is matching to Base+TotalTaxes+TotalFee')
			KeywordUtil.logInfo('Total price is appearing as ' + sTotalPrice +' And Base+totaltaxe+total fee is '+ sBaseTaxFee)
		}

		else

		{
			KeywordUtil.logInfo('Total price is appearing as ' + sTotalPrice +' And Base+totaltaxe+total fee is '+ sBaseTaxFee)
			KeywordUtil.markFailedAndStop('The Total Price is not equal to Base+TotalTaxes+TotalFee')

		}


	}


	@Keyword
	def OfferIdentifierValue(def ModifyOffer) {

		ErrorMessage(ModifyOffer)

		def slurper = new groovy.json.JsonSlurper()

		def sModifyOffer = slurper.parseText(ModifyOffer.getResponseBodyContent())

		def sOfferIdentifierValue = sModifyOffer.OfferListResponse.OfferID[0].Identifier.value

		GlobalVariable.sOfferIdentifierValue = sOfferIdentifierValue

		println('The OfferIdentifierValue is ' + GlobalVariable.sOfferIdentifierValue)


	}

	@Keyword
	def ExchSearchIdentifierValue(def SearchDomainPNR) {

		ErrorMessage(SearchDomainPNR)
		//ErrorMessagePostEligibility(SearchDomainPNR)

		def slurper = new groovy.json.JsonSlurper()

		def sSearchDomainPNR = slurper.parseText(SearchDomainPNR.getResponseBodyContent())

		def sExchSearchIdentifierValue = sSearchDomainPNR.CatalogOfferingsAirChangeResponse.CatalogOfferings.Identifier.value

		GlobalVariable.ExchSearchIdentifierValue = sExchSearchIdentifierValue

		println('The ExchSearchIdentifierValue is ' + GlobalVariable.ExchSearchIdentifierValue)







	}



	@Keyword
	def ReservationWorkBench(def ReservationWorkBench) {

		ErrorMessage(ReservationWorkBench)

		def slurper = new groovy.json.JsonSlurper()

		def sReservationWorkBench = slurper.parseText(ReservationWorkBench.getResponseBodyContent())

		def sIdentifierResWorkBench = sReservationWorkBench.ReservationResponse.Reservation.Identifier.value

		GlobalVariable.IdentifierResWorkBench = sIdentifierResWorkBench

		println('The IdentifierResWorkBench is ' + GlobalVariable.IdentifierResWorkBench)



		def DepartureCity = sReservationWorkBench.ReservationResponse.Reservation.Offer[0].Product[0].FlightSegment.findAll()

		//println(DepartureCity.size())



		if (GlobalVariable.sExecutionFlow == 'PNRGenerator') {


			if (DepartureCity.size()>1)

			{


				def sDepartureCity = sReservationWorkBench.ReservationResponse.Reservation.Offer[0].Product[0].FlightSegment[0].Flight.Departure.location
				def sArrivalCity = sReservationWorkBench.ReservationResponse.Reservation.Offer[0].Product[0].FlightSegment[1].Flight.Arrival.location

				println(sDepartureCity)
				GlobalVariable.sExch_From = sDepartureCity
				println(sArrivalCity)
				GlobalVariable.sExch_To = sArrivalCity


			}

			else

			{

				def sDepartureCity = sReservationWorkBench.ReservationResponse.Reservation.Offer[0].Product[0].FlightSegment[0].Flight.Departure.location
				def sArrivalCity = sReservationWorkBench.ReservationResponse.Reservation.Offer[0].Product[0].FlightSegment[0].Flight.Arrival.location
				println(sDepartureCity)
				GlobalVariable.sExch_From = sDepartureCity
				println(sArrivalCity)
				GlobalVariable.sExch_To = sArrivalCity
			}


		}

	}


	@Keyword
	def Eligibility(def Eligibility) {

		ErrorMessage(Eligibility)

		def slurper = new groovy.json.JsonSlurper()

		def sEligibility = slurper.parseText(Eligibility.getResponseBodyContent())

		//sChange = ((sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].Penalties.Change[0])['@type'])
		//def sExchangable = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].exchangeable

		def sResponse = sEligibility.findAll()

		sResponse.each {

			def sKeys =  " $it.key "

			sKeys = sKeys.toString().trim()



			if (sKeys == 'TicketChangeEligibilityListResponse') {


				def sExchangable = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibility[0].exchangeable

				if (sExchangable == 'All' || sExchangable == 'Some')

				{

					KeywordUtil.markPassed('Exchangeable Value is available in Response as :- '+ sExchangable)



					String sPTC = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibility[0].PassengerTypeCode

					if (sPTC.length() > 0 && sPTC != null) {

						KeywordUtil.markPassed('PTC Value is available in Response as:- '+ sPTC)

					} else {

						KeywordUtil.markFailed('Error Response returned or PTC Value is is not available')
					}


					String sTicketValue = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibility[0].Identifier.value

					if (sTicketValue.length() > 0 && sTicketValue != null) {

						KeywordUtil.markPassed('TicketNumber Value is available in Response as :-' + sTicketValue)


					} else {
						KeywordUtil.markFailed('Error Response returned or ticketNumber Value is is not available')

					}


					String sTraceID =sEligibility.TicketChangeEligibilityListResponse.traceId


					if (sTraceID.length() > 0 && sTraceID != null) {

						KeywordUtil.markPassed('traceid Value is available in Response as :-' + sTraceID)


					} else {
						KeywordUtil.markFailed('Error Response returned or traceid Value is is not available')

					}




					String sMaxChangeFee = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibility[0].Penalties.Change[0].Penalty[0].Amount.value

					if (sMaxChangeFee.length() > 0 && sMaxChangeFee != null) {

						KeywordUtil.markPassed('maximumChangeFee value  is available in Response as :- ' +sMaxChangeFee)


					} else {

						KeywordUtil.markFailed('Error Response returned or maximumChangeFee value  is is not available')

					}




					String sMinChangeFee = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibility[0].Penalties.Change[1].Penalty[0].Amount.value

					if (sMinChangeFee.length() > 0 && sMinChangeFee != null) {

						KeywordUtil.markPassed('minimumChangeFee value  is available in Response as :- ' +sMinChangeFee)


					} else {

						KeywordUtil.markFailed('Error Response returned or minimumChangeFee value  is is not available')

					}




				}

				else

				{


					KeywordUtil.markFailedAndStop('Error Response returned or Exchangeable Value is is not available')
					//KeywordUtil.logInfo('The PNR is not exchangeable')

				}


			}
		}






	}



	@Keyword
	def FormOfPaymentCode(def sFormofPayment) {




		if(sFormofPayment == 'CreditCard(AMEX)')

		{

			//println('the form of payment is ' +sFormofPayment)
			KeywordUtil.logInfo('The form of payment is ' +sFormofPayment)
			def PaymentResponse = WS.sendRequestAndVerify(findTestObject('FormOfPayment/8_Add FOP-CreditCard-Amex'))
			ErrorMessage(PaymentResponse)
			WS.verifyResponseStatusCode(PaymentResponse, 200)

			def AddPayment = WS.sendRequestAndVerify(findTestObject('FormOfPayment/9_Add Payment-CreditCard-Amex'))
			ErrorMessage(AddPayment)
			WS.verifyResponseStatusCode(AddPayment, 200)


		}

		else if (sFormofPayment == 'CreditCard(VISA)')


		{
			//println('the form of payment is ' +sFormofPayment)
			KeywordUtil.logInfo('The form of payment is ' +sFormofPayment)
			def PaymentResponse = WS.sendRequestAndVerify(findTestObject('FormOfPayment/8_Add FOP-CreditCard-Visa'))
			WS.verifyResponseStatusCode(PaymentResponse, 200)

			def AddPayment = WS.sendRequestAndVerify(findTestObject('FormOfPayment/9_Add Payment-CreditCard-Visa'))
			WS.verifyResponseStatusCode(AddPayment, 200)


		}

		else if (sFormofPayment == 'CreditCard(MasterCard)')


		{
			//println('the form of payment is ' +sFormofPayment)
			KeywordUtil.logInfo('The form of payment is ' +sFormofPayment)
			def PaymentResponse = WS.sendRequestAndVerify(findTestObject('FormOfPayment/8_Add FOP-CreditCard-Visa'))
			WS.verifyResponseStatusCode(PaymentResponse, 200)

			def AddPayment = WS.sendRequestAndVerify(findTestObject('FormOfPayment/9_Add Payment-CreditCard-Visa'))
			WS.verifyResponseStatusCode(AddPayment, 200)


		}


		else if (sFormofPayment == 'Cash')


		{
			//println('the form of payment is ' +sFormofPayment)
			KeywordUtil.logInfo('The form of payment is ' +sFormofPayment)
			def PaymentResponse = WS.sendRequestAndVerify(findTestObject('FormOfPayment/8_Add FOP-Cash'))
			ErrorMessage(PaymentResponse)
			WS.verifyResponseStatusCode(PaymentResponse, 200)

			def AddPayment = WS.sendRequestAndVerify(findTestObject('FormOfPayment/9_Add Payment-Cash'))
			ErrorMessage(AddPayment)
			WS.verifyResponseStatusCode(AddPayment, 200)


		}

		else


		{
			KeywordUtil.markFailedAndStop('The form of payment is not provided , kindly provide it in TestData sheet ')


		}







	}


	@Keyword
	def UpdateExcel(String Message) {


		//println('the message is '+ Message)
		def sFileName = GlobalVariable.sFileName

		def excelFile = new FileInputStream (new File(sFileName))
		//FileInputStream excelFile = new FileInputStream (new File(sFileName))


		Workbook workbook = new XSSFWorkbook(excelFile)
		//Workbook workbook = WorkbookFactory.create(excelFile);

		Sheet sheet = workbook.getSheet('Result')

		Font font = workbook.createFont();

		def style = workbook.createCellStyle();
		style.setFillBackgroundColor(IndexedColors.RED.getIndex())

		//font.setColor(HSSFColor.BLACK.index)
		//font.setFontName("Courier New");
		//font.setBold(true);
		//font.setUnderline(Font.U_SINGLE);


		def row = sheet.getRow(GlobalVariable.iTestRowNumber)
		Cell cell = row.getCell(15)//createCell(15)
		//def cell = row.getCell(15)//createCell(15)




		//println(Message)
		cell.setCellValue(Message)
		//cell.setCellStyle(style)

		def outFile = new FileOutputStream( new File(sFileName))
		workbook.write(outFile)

		excelFile.close()
		workbook.close()

		outFile.close()








	}

	@Keyword
	def PNRGeneratorPNR(String excelname) {

		def sfilename = LocateExcelFile(excelname)

		//println('the file name return is' +sfilename)

		//def excelFile = new File('S:/TestData/GeneratePNR_POC/PNRREPORT/25MAY22/Result111426AM.xlsx')

		def excelFile = new File(sfilename)

		def workbook = new XSSFWorkbook(excelFile)

		XSSFSheet sheet = workbook.getSheet('Result')

		def Totalrowcount = sheet.getLastRowNum()


		String sTestCaseName = GlobalVariable.sTestCaseName


		for (int j = 1; j < Totalrowcount+1; j++)

		{
			String Scenario = sheet.getRow(j).getCell(0);


			if (sTestCaseName == Scenario.toUpperCase())

			{


				String sPNR = sheet.getRow(j).getCell(1);

				String sTicketNumber = sheet.getRow(j).getCell(2);

				String sFlightDates = sheet.getRow(j).getCell(6)

				String sPCC = sheet.getRow(j).getCell(7)


				if(sTicketNumber.contains('Ticket Details Not Found'))

				{
					KeywordUtil.markFailedAndStop('The PNR is not ticketed for the scenario')

				}

				else

				{
					sTicketNumber = sTicketNumber.replaceAll("\\s","")
					GlobalVariable.PNR = sPNR
					GlobalVariable.sFirstTicket = sTicketNumber
					GlobalVariable.sOldTicket = sTicketNumber
					GlobalVariable.sPCC = sPCC
					//println (sPNR)
					KeywordUtil.logInfo('The PNRGenerator PNR is '+sPNR)
					//println (sTicketNumber)
					KeywordUtil.logInfo('The Ticket Number is '+sTicketNumber)
					//println (Scenario)
					KeywordUtil.logInfo('The Test Scenarios is '+Scenario)
					KeywordUtil.logInfo('The PCC is '+sPCC)
					GlobalVariable.iTestRowNumber = j

					//println (GlobalVariable.sFlightDates)
					FlightDates(sFlightDates)
					def sAccessGroup

					if(GlobalVariable.sPCC == "O1P")

					{


						sAccessGroup = "19CF434B-3D98-4312-B626-09E427FC1691"
						GlobalVariable.sAccessGroup = sAccessGroup
						KeywordUtil.logInfo('The PCC "' + GlobalVariable.sPCC +'" AccessGroup ID is ' + GlobalVariable.sAccessGroup)


					}

					else if (GlobalVariable.sPCC == "79JP")

					{
						sAccessGroup = "7C7ED10A-EEBC-4468-B499-879DE63F1B7D"
						GlobalVariable.sAccessGroup = sAccessGroup
						KeywordUtil.logInfo('The PCC "' + GlobalVariable.sPCC +'" AccessGroup ID is ' + GlobalVariable.sAccessGroup)


					}

					else if (GlobalVariable.sPCC == "62HA")

					{
						sAccessGroup = "F976DEE8-BB4C-4912-83AA-B3D89BDC04C7"
						GlobalVariable.sAccessGroup = sAccessGroup
						KeywordUtil.logInfo('The PCC "' + GlobalVariable.sPCC +'" AccessGroup ID is ' + GlobalVariable.sAccessGroup)


					}

					else if (GlobalVariable.sPCC == "51MG")

					{
						sAccessGroup = "898C55E6-0DD6-4B4D-8D56-0B96033CB150"
						GlobalVariable.sAccessGroup = sAccessGroup
						KeywordUtil.logInfo('The PCC "' + GlobalVariable.sPCC +'" AccessGroup ID is ' + GlobalVariable.sAccessGroup)


					}

					break

				}


			}





		}





	}



	@Keyword
	def LocateExcelFile(String excelname) {

		def excelfilename = excelname.trim()

		//def excelfilename = 'Result80854PM'

		if (excelfilename.contains('.xlsx'))

		{

			excelfilename = excelfilename.replace('.xlsx', "")

		}

		//excelfilename = excelfilename.toUpperCase()

		def sFlag

		def sDate = new Date()

		sDate = sDate.format("ddMMMYY").toUpperCase()

		//println('the departure is ' + sDate)

		for (int i = 1; i < 20; i++)

		{

			if(i >= 18)

			{
				//println('The excelfile is not located,kindly recheck it.')
				KeywordUtil.markFailedAndStop('The excelfile is not located,kindly recheck it.')
				break

			}

			def dir = new File('S:/TestData/GeneratePNR_POC/PNRREPORT/'+sDate+'/')

			if(dir.exists()) {


				def files = []
				dir.traverse(type: FILES, maxDepth: 0)

				{files.add(it)}

				for (int j = 0; j < files.size(); j++)

				{

					String sFileName = files.getAt(j)

					//sFileName = sFileName.toUpperCase()
					//println('the filename exist '+ sFileName)
					//sTestCaseID = sTestCaseID.substring((sTestCaseID.lastIndexOf("/").toInteger()) + 1)

					if(sFileName.contains(excelfilename))

					{


						sFileName = sFileName.trim()

						//sFileName = sFileName.substring((sFileName.lastIndexOf("/").toInteger()) + 1)
						println('the filename exist '+ sFileName)

						sFlag = true

						GlobalVariable.sFileName = sFileName

						return sFileName



						break;

					}

				}
			}

			if (sFlag == true)
			{

				break;


			}

			else

			{

				Date date = new Date().minus(i)

				sDate = date.format("ddMMMYY").toUpperCase()

				//println('the new folder name is'+ sDate)



			}




		}



		//Date date = new Date().plus(30)

		//To Substract days:

		//Date date = new Date().minus(30)

		//S:\TestData\GeneratePNR_POC\PNRREPORT\25MAY22\Result111426AM.xlsx
		//S:\TestData\GeneratePNR_POC\PNRREPORT\25MAY22\Result80854PM.xlsx


	}


	@Keyword
	def FlightDates(String sFlightDates) {

		sFlightDates = sFlightDates.trim()
		sFlightDates = sFlightDates.replace(" ","")

		println(sFlightDates);
		def OriginalDepDate
		def OriginalRetDate

		if(sFlightDates.contains(","))

		{

			String[] Dates = sFlightDates.split(",")
			//int FltDatesSize = Dates.size()

			//println FltDatesSize
			OriginalDepDate = Dates.first()
			//println(OriginalDepDate);
			OriginalRetDate =  Dates.last()
			//println(OriginalRetDate);

			if (GlobalVariable.sExchDepDay.any().value  && OriginalDepDate.any().value)

			{
				int sExch_DepartureDays = GlobalVariable.sExchDepDay as int
				Date date = Date.parse("ddMMM", OriginalDepDate)
				def sNewDepDate = date.plus(sExch_DepartureDays).format("yyyy-MM-dd").toUpperCase()
				int year = Calendar.getInstance().get(Calendar.YEAR);
				sNewDepDate = year + sNewDepDate.substring(4);
				GlobalVariable.sExch_departureDate = sNewDepDate
				KeywordUtil.logInfo('Exchange Dep Date for PNRGenerator PNR is' + sNewDepDate)

			}


			if (GlobalVariable.sExchRetDay.any().value && OriginalRetDate.any().value)

			{

				int sExch_RetnDays = GlobalVariable.sExchRetDay as int
				Date date = Date.parse("ddMMM", OriginalRetDate)
				def sNewRetDate = date.plus(sExch_RetnDays).format("yyyy-MM-dd").toUpperCase()
				int year = Calendar.getInstance().get(Calendar.YEAR);
				sNewRetDate = year + sNewRetDate.substring(4);
				KeywordUtil.logInfo('Exchange Ret Date for PNRGenerator PNR is' + sNewRetDate)
				GlobalVariable.sExch_RetnDate = sNewRetDate

			}

		}

		else

		{

			if (GlobalVariable.sExchDepDay.any().value ) {
				OriginalDepDate = sFlightDates
				int sExch_DepartureDays = GlobalVariable.sExchDepDay as int
				Date date = Date.parse("ddMMM", OriginalDepDate)
				def sNewDepDate = date.plus(sExch_DepartureDays).format("yyyy-MM-dd").toUpperCase()
				int year = Calendar.getInstance().get(Calendar.YEAR);
				sNewDepDate = year + sNewDepDate.substring(4);
				GlobalVariable.sExch_departureDate = sNewDepDate
				KeywordUtil.logInfo('Exchange Dep Date for PNRGenerator PNR ' + sNewDepDate)

			}

			else
			{

				KeywordUtil.markFailedAndStop('Kindly provide Exchange Dep Date to exchange PNR in data sheet ')

			}

		}





		//println(sExch_DepartureDays)
		//println(sExch_RetnDays)







	}



	@Keyword
	def TripChangeFlow() {


		//-----------------------------------------GetEligibility-------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Eligibility Request----')

		def sEligibility = WS.sendRequest(findTestObject('OneWayTrip/S_GetEligibilityRequest'))

		WS.verifyResponseStatusCode(sEligibility, 200)

		Eligibility(sEligibility)

		//----------------------------------------ReservationWorkBench---------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing ReservationWorkBench Request----')

		def sReservationWorkBench = WS.sendRequest(findTestObject('OneWayTrip/T_Reservationworkbench'))

		WS.verifyResponseStatusCode(sReservationWorkBench, 200)

		ReservationWorkBench(sReservationWorkBench)

		//------------------------------------------SearchDomainPNR-------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Exchange Search Request----')

		def SearchDomainPNR = WS.sendRequestAndVerify(findTestObject('OneWayTrip/U-SearchDomainPNR'))

		ExchSearchIdentifierValue(SearchDomainPNR)

		WS.verifyResponseStatusCode(SearchDomainPNR, 200)

		//--------------------------------------------Modify Offer----------------------------------------------------------------

		KeywordUtil.logInfo('----Processing ModifyOffer Request----')

		def ModifyOffer = WS.sendRequest(findTestObject('OneWayTrip/V_Modify Offer'))

		OfferIdentifierValue(ModifyOffer)

		WS.verifyResponseStatusCode(ModifyOffer, 200)

		//--------------------------------------------Add Payment------------------------------------------------------------------

		KeywordUtil.logInfo('----Processing Add Payment Request----')

		def AddPayment = WS.sendRequest(findTestObject('OneWayTrip/X_Add Payment PNR'))

		WS.verifyResponseStatusCode(AddPayment, 200)

		//--------------------------------------------------------------------------------------------------------------------------

		KeywordUtil.logInfo('----Processing Etkt Reissuance Request----')

		def sReservationCommit = WS.sendRequest(findTestObject('OneWayTrip/Y_Reservation Commit'))

		ReservationCommit(sReservationCommit)

		WS.verifyResponseStatusCode(sReservationCommit, 200)




	}

	@Keyword
	def TripSearchFlowOneWay() {

		KeywordUtil.logInfo('----------------TripSearch Flow-----------------')

		//--------------------------------------Search---------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Air Search Request for TripSearch----')

		//println(GlobalVariable.sTestCaseID)

		String sTestCaseID = GlobalVariable.sTestCaseID

		if(sTestCaseID.contains('9PASSENGERS')){

			def SearchResponse = WS.sendRequestAndVerify(findTestObject('9Passengers/1_Search Request_9Passengers'))

			SearchCatalogueIDAndProductID(SearchResponse)

			WS.verifyResponseStatusCode(SearchResponse, 200)


		}else {


			def SearchResponse = WS.sendRequestAndVerify(findTestObject('OneWayTrip/1_Search Request'))

			SearchCatalogueIDAndProductID(SearchResponse)

			WS.verifyResponseStatusCode(SearchResponse, 200)
		}
		//----------------------------------Price-------------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Price Request for TripSearch----')

		def PriceResponse = WS.sendRequestAndVerify(findTestObject('OneWayTrip/2_Price Request'))

		FareValidation(PriceResponse)

		WS.verifyResponseStatusCode(PriceResponse, 200)

		//println('Value of Search Identifier Token is ' + sPriceResponse.toString())
		//-------------------------------------Initiate Workbench--------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing InitiateWorkBenchResponse Request for TripSearch----')

		def InitiateWorkBenchResponse = WS.sendRequest(findTestObject('OneWayTrip/3_TS Intiate_Workbench'))

		InitiateWorkBench(InitiateWorkBenchResponse)

		WS.verifyResponseStatusCode(InitiateWorkBenchResponse, 200)

		//-------------------------------------Add_Offer Catalogue-----------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Add_Offer_Catalogue Request for TripSearch----')

		def Add_Offer_Catalogue = WS.sendRequestAndVerify(findTestObject('OneWayTrip/4_Add_Offer Catalogue'))

		WS.verifyResponseStatusCode(Add_Offer_Catalogue, 200)

		//-----------------------------------------Add_Traveler------------------------------------------------------------------------------

		KeywordUtil.logInfo('----Processing Add_Traveler Request for TripSearch----')

		if(sTestCaseID.contains('9PASSENGERS')) {
			//if(GlobalVariable.sTestCaseID == '9PASSENGERS') {

			for (def i = 1; i < 10; i++) {

				def sResponse = WS.sendRequestAndVerify(findTestObject(('9Passengers/5_Add_Traveler_9Passengers_' + i) + 'Pax'))

				AddTravellerErrorMessage(sResponse)
			}

		}else {


			def Add_Traveler = WS.sendRequestAndVerify(findTestObject('OneWayTrip/5_Add_Traveler'))

			WS.verifyResponseStatusCode(Add_Traveler, 200)

		}
		//-----------------------------------------Commit--------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Commit(Create PNR) Request for TripSearch----')

		def sCommit = WS.sendRequest(findTestObject('Object Repository/OneWayTrip/6_Commit'))

		Commit(sCommit)

		WS.verifyResponseStatusCode(sCommit, 200)

		//-----------------------------------------InitiateWorkBenchWithPNR---------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing WorkBenchWithPNR Request for TripSearch----')

		def sWorkBenchWithPNR = WS.sendRequest(findTestObject('OneWayTrip/7_Intiate workbench with PNR'))

		WorkBenchWithPNR(sWorkBenchWithPNR)

		WS.verifyResponseStatusCode(sWorkBenchWithPNR, 200)

		//-----------------------------------------Add_FoP---------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Add FormofPayment Request for TripSearch----')

		//AddFoP = WS.sendRequest(findTestObject('OneWayTrip/8_Add FOP'))
		//WS.verifyResponseStatusCode(AddFoP, 200)
		FormOfPaymentCode(GlobalVariable.sFormOfPayment)

		//-----------------------------------------Add_Payment-----------------------------------------------------------------------------------
		//AddPayment = WS.sendRequest(findTestObject('OneWayTrip/9_Add Payment'))

		//WS.verifyResponseStatusCode(AddPayment, 200)

		//----------------------------------------Ticket_Issuance---------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Etkt Issuance Request for TripSearch----')

		def TicketIssuance = WS.sendRequest(findTestObject('OneWayTrip/R_Ticket Issuance'))

		WS.verifyResponseStatusCode(TicketIssuance, 200)

		TicketNumber(TicketIssuance)

		//TripChangeFlow()

	}


	@Keyword
	def TripSearchFlow_ReturnTrip() {


		KeywordUtil.logInfo('----TripSearch Flow----')

		//--------------------------------------Search---------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Search Request for TripSearch----')

		def SearchResponse = WS.sendRequestAndVerify(findTestObject('ReturnTrip_Payload/1_Search_ReturnTrip'))

		SearchCatalogueIDAndProductID(SearchResponse)

		WS.verifyResponseStatusCode(SearchResponse, 200)

		//----------------------------------Price-------------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Price Request for TripSearch----')

		def PriceResponse = WS.sendRequestAndVerify(findTestObject('ReturnTrip_Payload/2_Price_ReturnTrip'))

		FareValidation(PriceResponse)

		WS.verifyResponseStatusCode(PriceResponse, 200)

		//println('Value of Search Identifier Token is ' + sPriceResponse.toString())
		//-------------------------------------Initiate Workbench--------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing InitiateWorkBenchResponse Request for TripSearch----')

		def InitiateWorkBenchResponse = WS.sendRequest(findTestObject('ReturnTrip_Payload/3_TS Intiate_Workbench_Return'))

		InitiateWorkBench(InitiateWorkBenchResponse)

		WS.verifyResponseStatusCode(InitiateWorkBenchResponse, 200)

		//-------------------------------------Add_Offer Catalogue-----------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Add_Offer_Catalogue Request for TripSearch----')

		def Add_Offer_Catalogue = WS.sendRequestAndVerify(findTestObject('ReturnTrip_Payload/4_Add_Offer Catalogue_Return'))

		WS.verifyResponseStatusCode(Add_Offer_Catalogue, 200)

		//-----------------------------------------Add_Traveler------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Add_Traveler Request for TripSearch----')

		def Add_Traveler = WS.sendRequestAndVerify(findTestObject('ReturnTrip_Payload/5_Add_Traveler_Return'))

		WS.verifyResponseStatusCode(Add_Traveler, 200)

		//-----------------------------------------Commit--------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Commit(Create PNR) Request for TripSearch----')

		def sCommit = WS.sendRequest(findTestObject('ReturnTrip_Payload/6_Commit_Return'))

		Commit(sCommit)

		WS.verifyResponseStatusCode(sCommit, 200)

		//-----------------------------------------InitiateWorkBenchWithPNR---------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing WorkBenchWithPNR Request for TripSearch----')

		def sWorkBenchWithPNR = WS.sendRequest(findTestObject('ReturnTrip_Payload/7_Intiate workbench with PNR_Return'))

		WorkBenchWithPNR(sWorkBenchWithPNR)

		WS.verifyResponseStatusCode(sWorkBenchWithPNR, 200)

		//-----------------------------------------Add_FoP---------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Add FormofPayment Request for TripSearch----')

		//AddFoP = WS.sendRequest(findTestObject('OneWayTrip/8_Add FOP'))
		//WS.verifyResponseStatusCode(AddFoP, 200)
		FormOfPaymentCode(GlobalVariable.sFormOfPayment)

		//-----------------------------------------Add_Payment-----------------------------------------------------------------------------------
		//AddPayment = WS.sendRequest(findTestObject('OneWayTrip/9_Add Payment'))

		//WS.verifyResponseStatusCode(AddPayment, 200)

		//----------------------------------------Ticket_Issuance---------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Etkt Issuance Request for TripSearch----')

		def TicketIssuance = WS.sendRequest(findTestObject('ReturnTrip_Payload/R_Ticket Issuance_Return'))

		WS.verifyResponseStatusCode(TicketIssuance, 200)

		TicketNumber(TicketIssuance)

		//TripChangeFlow()



	}



	@Keyword
	def TripSearchFlow_CircleTrip() {


		KeywordUtil.logInfo('----TripSearch Flow----')

		//--------------------------------------Search---------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Search Request for TripSearch----')

		def SearchResponse = WS.sendRequestAndVerify(findTestObject('CircleTrip_Payload/1_Search_CT'))

		SearchCatalogueIDAndProductID(SearchResponse)

		WS.verifyResponseStatusCode(SearchResponse, 200)

		//----------------------------------Price-------------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Price Request for TripSearch----')

		def PriceResponse = WS.sendRequestAndVerify(findTestObject('CircleTrip_Payload/2_Price_CT'))

		FareValidation(PriceResponse)

		WS.verifyResponseStatusCode(PriceResponse, 200)

		//println('Value of Search Identifier Token is ' + sPriceResponse.toString())
		//-------------------------------------Initiate Workbench--------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing InitiateWorkBenchResponse Request for TripSearch----')

		def InitiateWorkBenchResponse = WS.sendRequest(findTestObject('CircleTrip_Payload/3_TS Intiate_Workbench_CT'))

		InitiateWorkBench(InitiateWorkBenchResponse)

		WS.verifyResponseStatusCode(InitiateWorkBenchResponse, 200)

		//-------------------------------------Add_Offer Catalogue-----------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Add_Offer_Catalogue Request for TripSearch----')

		def Add_Offer_Catalogue = WS.sendRequestAndVerify(findTestObject('CircleTrip_Payload/4_Add_Offer Catalogue_CT'))

		WS.verifyResponseStatusCode(Add_Offer_Catalogue, 200)

		//-----------------------------------------Add_Traveler------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Add_Traveler Request for TripSearch----')

		def Add_Traveler = WS.sendRequestAndVerify(findTestObject('CircleTrip_Payload/5_Add_Traveler_CT'))

		WS.verifyResponseStatusCode(Add_Traveler, 200)

		//-----------------------------------------Commit--------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Commit(Create PNR) Request for TripSearch----')

		def sCommit = WS.sendRequest(findTestObject('CircleTrip_Payload/6_Commit_CT'))

		Commit(sCommit)

		WS.verifyResponseStatusCode(sCommit, 200)

		//-----------------------------------------InitiateWorkBenchWithPNR---------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing WorkBenchWithPNR Request for TripSearch----')

		def sWorkBenchWithPNR = WS.sendRequest(findTestObject('CircleTrip_Payload/7_Intiate workbench with PNR_CT'))

		WorkBenchWithPNR(sWorkBenchWithPNR)

		WS.verifyResponseStatusCode(sWorkBenchWithPNR, 200)

		//-----------------------------------------Add_FoP---------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Add FormofPayment Request for TripSearch----')

		//AddFoP = WS.sendRequest(findTestObject('OneWayTrip/8_Add FOP'))
		//WS.verifyResponseStatusCode(AddFoP, 200)
		FormOfPaymentCode(GlobalVariable.sFormOfPayment)

		//-----------------------------------------Add_Payment-----------------------------------------------------------------------------------
		//AddPayment = WS.sendRequest(findTestObject('OneWayTrip/9_Add Payment'))

		//WS.verifyResponseStatusCode(AddPayment, 200)

		//----------------------------------------Ticket_Issuance---------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Etkt Issuance Request for TripSearch----')

		def TicketIssuance = WS.sendRequest(findTestObject('CircleTrip_Payload/R_Ticket Issuance_CT'))

		WS.verifyResponseStatusCode(TicketIssuance, 200)

		TicketNumber(TicketIssuance)






		//-----------------------------------------GetEligibility-------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing Eligibility Request----')

		def sEligibility = WS.sendRequest(findTestObject('CircleTrip_Payload/S_GetEligibilityRequest_CT'))

		Eligibility(sEligibility)

		WS.verifyResponseStatusCode(sEligibility, 200)



		//----------------------------------------ReservationWorkBench---------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing ReservationWorkBench Request----')

		def sReservationWorkBench = WS.sendRequest(findTestObject('CircleTrip_Payload/T_Reservationworkbench_CT'))

		WS.verifyResponseStatusCode(sReservationWorkBench, 200)

		ReservationWorkBench(sReservationWorkBench)

		//------------------------------------------SearchDomainPNR-------------------------------------------------------------------------------------
		KeywordUtil.logInfo('----Processing SearchDomain Request----')

		def SearchDomainPNR = WS.sendRequestAndVerify(findTestObject('CircleTrip_Payload/U-SearchDomainPNR_CT'))

		ExchSearchIdentifierValue(SearchDomainPNR)

		WS.verifyResponseStatusCode(SearchDomainPNR, 200)

		//--------------------------------------------Modify Offer----------------------------------------------------------------

		KeywordUtil.logInfo('----Processing ModifyOffer Request----')

		def ModifyOffer = WS.sendRequest(findTestObject('CircleTrip_Payload/V_Modify Offer_CT'))

		OfferIdentifierValue(ModifyOffer)

		WS.verifyResponseStatusCode(ModifyOffer, 200)

		//--------------------------------------------Add Payment------------------------------------------------------------------

		KeywordUtil.logInfo('----Processing Add Payment Request----')

		def AddPayment = WS.sendRequest(findTestObject('CircleTrip_Payload/X_Add Payment PNR_CT'))

		WS.verifyResponseStatusCode(AddPayment, 200)

		//--------------------------------------------------------------------------------------------------------------------------

		KeywordUtil.logInfo('----Processing Etkt Reissuance Request----')

		def sReservationCommit = WS.sendRequest(findTestObject('CircleTrip_Payload/Y_Reservation Commit_CT'))

		ReservationCommit(sReservationCommit)

		WS.verifyResponseStatusCode(sReservationCommit, 200)



	}




	@Keyword
	def TestExecution() {

		String sComponentLevelTest =  GlobalVariable.sComponentLevelTest
		String sComponentTestType = GlobalVariable.sComponentTestType





		if ((GlobalVariable.sExecutionFlow == 'TripSearchFlow') && sComponentLevelTest.trim().contains('Component_Level_Test') )
		{

			String sTestCaseID =  GlobalVariable.sTestCaseID
			//println('sTestCaseID'+sTestCaseID)

			KeywordUtil.logInfo('Its a '+GlobalVariable.sComponentLevelTest)
			//println('sExecutionFlow '+GlobalVariable.sExecutionFlow)
			//println('sTestCaseID '+GlobalVariable.sTestCaseID)

			if(sTestCaseID.contains('SINGLESEGMENT') || (sTestCaseID.contains('ONEWAY')) || (sTestCaseID.contains('OneWay')))  {

				TripSearchFlowOneWay()

			}else if(sTestCaseID.contains('RETURNTRIP')) {

				TripSearchFlow_ReturnTrip()
			}
			else if(sTestCaseID.contains('CIRCLETRIP')) {

				TripSearchFlow_CircleTrip()
			}



		}



		else if (GlobalVariable.sExecutionFlow == 'TripSearchFlow')

		{

			String sTestCaseID =  GlobalVariable.sTestCaseID

			if(sTestCaseID.contains('SINGLESEGMENT') || (sTestCaseID.contains('ONEWAY'))) {

				TripSearchFlowOneWay()
				TripChangeFlow()

			}else if(sTestCaseID.contains('RETURNTRIP')|| (sTestCaseID.contains('ROUND')) ) {

				TripSearchFlow_ReturnTrip()
				TripChangeFlow()
			}
			else if(sTestCaseID.contains('CIRCLETRIP')) {

				TripSearchFlow_CircleTrip()
				TripChangeFlow()
			}






		}





		else if (GlobalVariable.sExecutionFlow == 'PNRGenerator' && sComponentLevelTest.contains('Component_Level_Test') )

		{

			KeywordUtil.logInfo('Its a '+GlobalVariable.sComponentLevelTest)
			PNRGeneratorPNR(GlobalVariable.sFileName)
			//TripChangeFlow()


		}


		else if (GlobalVariable.sExecutionFlow == 'PNRGenerator')

		{


			//CustomKeywords.'methods.APIKeywords.ManualPNR'(GlobalVariable.sFileName)

			//CustomKeywords.'methods.APIKeywords.TripChangeFlow'()

			PNRGeneratorPNR(GlobalVariable.sFileName)
			TripChangeFlow()




		}

		else if (GlobalVariable.sExecutionFlow == 'TerminalPNR' && sComponentLevelTest.contains('Component_Level_Test') )

		{


			KeywordUtil.logInfo('Its a '+GlobalVariable.sComponentLevelTest)
			TerminalPNR()
			//TripChangeFlow()


		}

		else if (GlobalVariable.sExecutionFlow == 'TerminalPNR')

		{


			TerminalPNR()
			TripChangeFlow()




		}


	}




	@Keyword
	def TerminalPNROld() {

		def pcc

		def customerProfileId
		def sColumnvalue

		String sFirstAdtPassenger = 'Test/Uma'
		String sEnterPassengerName = 'N.'+sFirstAdtPassenger+'*P-ADT'
		String sPhone = 'p.nnnn'
		String sTicketTimelimit = 't.t/'
		String sReceivedfrom = 'r.p'
		String AddCommit = 'er'
		String IgnoreandRetreive = 'ir'
		String SSRDocs = 'SI.P1/SSRDOCSVAHK1/////12JUL76/M//'+sFirstAdtPassenger
		GlobalVariable.sDepDate
		GlobalVariable.sRetDate
		GlobalVariable.sFrom
		GlobalVariable.sTo
		String availabilityCheck = 'A'+GlobalVariable.sDepDate+GlobalVariable.sFrom+GlobalVariable.sTo+'/'+GlobalVariable.sCarrier+'#'
		println(availabilityCheck)



		//GlobalVariable.Pcc=pcc
		//GlobalVariable.sCustomerProfileId=customerProfileId

		WS.sendRequestAndVerify(findTestObject('Object Repository/Hcaservices/HCA_login'))

		//WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_login', [('pcc') : GlobalVariable.Pcc, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))


		def SerialNumber = findTestData('Data Files/TerminalData').getValue('SerialNumber', 1)

		def columnNumbers = findTestData('Data Files/TerminalData').columnNumbers

		def EmptyPnr = findTestData('Data Files/TerminalData').getValue('EmptyPnr', 1)




		//for (def columnSize = 1; columnSize <= columnNumbers; columnSize++) {

		for (def columnSize = 2; columnSize <= 17; columnSize++) {

			ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

			def xmlSlurpur = new XmlSlurper()

			def xmlResponse = xmlSlurpur.parseText(response.responseBodyContent)

			sColumnvalue = findTestData('Data Files/TerminalData').getValue(columnSize, 1)

			//println('xmlResponse is '+xmlResponse)
			//println('columnSize is '+columnSize)
			//println('serial no'+SerialNumber)
			//println('columnValue is '+sColumnvalue)




			//if ((columnSize == 1)||(findTestData('Data Files/TerminalData').getValue(columnSize, Integer.parseInt(SerialNumber)).equalsIgnoreCase("NA"))) {

			//	continue

			//}






			if (columnSize == 2) {

				String[] searchComments = findTestData('Data Files/TerminalData').getValue(columnSize, Integer.parseInt(SerialNumber)).toString().split(':')

				for (String availability : searchComments) {

					String[] searchandItinerary = availability.split(',')

					availabilityCheck = (searchandItinerary[0]).toString()

					def dynamicDates = new com.tvlport.automation.request.DynamicDates()

					if (availabilityCheck.toString().contains("date")) {
						String date = dynamicDates.getDynamicDate(availabilityCheck.substring(1, 6))

						println('date\t' + date)

						availabilityCheck = ((availabilityCheck.substring(0, 1) + date) + availabilityCheck.substring(6))
					}

					WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
						, ('Dynamiccomment') : availabilityCheck, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

					WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
						, ('Dynamiccomment') : searchandItinerary[1], ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))
				}
			}



			WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
				, ('Dynamiccomment') : findTestData('Data Files/TerminalData').getValue(columnSize, Integer.parseInt(SerialNumber)), ('pcc') : GlobalVariable.sPCC
				, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

			if (xmlResponse.Body.CommandRS.HCAResponse.HCAA_Data.HCAA_Payload.HCAATerm.toString().contains("CONFIRM SEGMENT")) {


				WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
					, ('Dynamiccomment') : 'er', ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))



			}

			if (findTestData('Data Files/TerminalData').getValue(columnSize, Integer.parseInt(SerialNumber)).equalsIgnoreCase("ir")) {

				// String retrievePnr = xmlResponse.Body.CommandRS.HCAResponse.HCAA_Data.HCAA_Payload.HCAATerm

				println('The PNR Contents are   \n' + xmlResponse.Body.CommandRS.HCAResponse.HCAA_Data.HCAA_Payload.HCAATerm)

				GlobalVariable.PNR = xmlResponse.Body.CommandRS.HCAResponse.HCAA_Data.HCAA_Payload.HCAATerm.toString().substring(
						0, 6)

				println('The PNR is '+ GlobalVariable.PNR)



			}


			if(columnSize == 17)	{

				def PNR = '*'+GlobalVariable.PNR
				println('The PNR with * is '+ PNR)

				WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
					, ('Dynamiccomment') : PNR, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

				WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
					, ('Dynamiccomment') : findTestData('Data Files/TerminalData').getValue(columnSize, Integer.parseInt(SerialNumber)), ('pcc') : GlobalVariable.sPCC
					, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

				response = WSResponseManager.getInstance().getCurrentResponse()

				xmlSlurpur = new XmlSlurper()

				xmlResponse = xmlSlurpur.parseText(response.responseBodyContent)


				//println('The Ticket Contents are   \n' + xmlResponse)

				String sTicket = xmlResponse.Body.CommandRS.HCAResponse.HCAA_Data.HCAA_Payload.HCAATerm.toString().substring(5, 22)
				sTicket = sTicket.trim()
				sTicket = sTicket.replace(" ", "")
				GlobalVariable.sOldTicket = sTicket
				println('The Ticket Number * is '+ GlobalVariable.sOldTicket )

			}


			if(columnSize==16&&EmptyPnr.toString().equalsIgnoreCase("true"))
			{
				WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
					, ('Dynamiccomment') : "*"+GlobalVariable.PNR.toString(), ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))
				WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
					, ('Dynamiccomment') : "IR", ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))
				WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
					, ('Dynamiccomment') : "X1", ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))
				WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
					, ('Dynamiccomment') : "r.p", ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))
				WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
					, ('Dynamiccomment') : "er", ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))
				WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
					, ('Dynamiccomment') : "er", ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))
			}




		}

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_logout', [('SessionToken') : GlobalVariable.hca_ExtSessiontoken, ('pcc') : GlobalVariable.sPCC
			, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

	}





	@Keyword
	def TerminalPNR() {

		def availabilityCheck = 'A'+GlobalVariable.sDepDate+GlobalVariable.sFrom+GlobalVariable.sTo+'/'+GlobalVariable.sCarrier+'#'
		def sSeatSell = 'N'+GlobalVariable.sPassengerCount+GlobalVariable.sClassOfService+1+'*'
		def ReturnAvailability = 'A'+GlobalVariable.sRetDate+GlobalVariable.sTo+GlobalVariable.sFrom+'/'+GlobalVariable.sCarrier+'#'
		def CircleAvailability1 = 'A'+GlobalVariable.sCircleDate+GlobalVariable.sTo+GlobalVariable.sCircleTripApt+'/'+GlobalVariable.sCarrier+'#'
		def CircleAvailability2 = 'A'+GlobalVariable.sRetDate+GlobalVariable.sCircleTripApt+GlobalVariable.sFrom+'/'+GlobalVariable.sCarrier+'#'

		//println(availabilityCheck)
		//println(sSeatSell)
		String City = GlobalVariable.sAgencyCity
		String sPCC = GlobalVariable.sPCC
		String sFirstAdtPassenger = 'Test/One'
		String sEnterPassengerName = 'N.'+sFirstAdtPassenger+'*P-ADT'
		String sPhone = 'p.nnnn'
		String sTicketTimelimit = 't.t/'
		String sReceivedfrom = 'r.p'
		String AddCommit = 'er'
		String IgnoreandRetreive = 'ir'
		String SSRDocs = 'SI.P1/SSRDOCS'+GlobalVariable.sCarrier+'HK1/////12JUL80/M//'+sFirstAdtPassenger
		def FareEntry = GlobalVariable.FareEntry
		def	sFoP
		def sCreditCard


		//		Date now = new Date()
		//		int sNYear = now.getYear() + 1900
		//		def Year = String.valueOf(sNYear).substring(2);
		//		int sNewYear = Integer.parseInt(Year)+2
		//		println(sNewYear)

		def sDate = new Date()
		sDate = sDate.format("YY")
		int sNYear = Integer.parseInt(sDate)+2
		//println(sNYear)


		if(GlobalVariable.sFormOfPayment=='Cash') {

			sFoP = 'F.S'

		}

		else if (GlobalVariable.sFormOfPayment == 'CHECK') {

			sFoP = 'F.CK'
		}

		else if (GlobalVariable.sFormOfPayment == 'CreditCard(VISA)') {

			sCreditCard = 'VI4444333322221111/D12'+sNYear+'/A12345'
			sFoP = 'F.'+sCreditCard
		}

		else if (GlobalVariable.sFormOfPayment == 'CreditCard(AMEX)') {

			sCreditCard = 'AX370000000000028/D12'+sNYear+'/E03/A12345'
			sFoP = 'F.'+sCreditCard
		}
		else if (GlobalVariable.sFormOfPayment == 'CreditCard(MasterCard)') {

			sCreditCard = 'CA5573470000004193/D12'+sNYear+'/123/A12345'
			sFoP = 'F.'+sCreditCard
		}

		String sTestCaseID =  GlobalVariable.sTestCaseID


		//-----------------------------------------------------------------------------------------


		WS.sendRequestAndVerify(findTestObject('Object Repository/Hcaservices/HCA_login'))

		ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

		def xmlSlurpur = new XmlSlurper()

		def xmlResponse = xmlSlurpur.parseText(response.responseBodyContent)


		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : 'SEM/PR', ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))


		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : 'HMCT-'+City+'/I', ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : 'HMCT-'+City+'/A', ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : 'HMCT-'+City+'/FD', ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : 'SEM/'+sPCC+'/AG', ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		KeywordUtil.logInfo('--------Processing Availability Entry-----------')

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : availabilityCheck, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		KeywordUtil.logInfo('--------Processing Sell Entry-----------')

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : sSeatSell, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))


		if(sTestCaseID.contains('RETURN') || sTestCaseID.contains('ROUND'))

		{


			KeywordUtil.logInfo('--------Processing Return Availability Entry-----------')

			WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
				, ('Dynamiccomment') : ReturnAvailability, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

			KeywordUtil.logInfo('--------Processing Sell Entry-----------')

			WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
				, ('Dynamiccomment') : sSeatSell, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))



		}


		if(sTestCaseID.contains('CIRCLE'))

		{


			KeywordUtil.logInfo('--------Processing Circle Availability Entry-----------')

			WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
				, ('Dynamiccomment') : CircleAvailability1, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

			KeywordUtil.logInfo('--------Processing Sell Entry-----------')

			WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
				, ('Dynamiccomment') : sSeatSell, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

			WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
				, ('Dynamiccomment') : CircleAvailability2, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

			KeywordUtil.logInfo('--------Processing Sell Entry-----------')

			WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
				, ('Dynamiccomment') : sSeatSell, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))




		}


		KeywordUtil.logInfo('--------Processing Passenger Name Entry-----------')

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : sEnterPassengerName, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : sPhone, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : sTicketTimelimit, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : sReceivedfrom, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : AddCommit, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : IgnoreandRetreive, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : IgnoreandRetreive, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		response = WSResponseManager.getInstance().getCurrentResponse()

		xmlSlurpur = new XmlSlurper()

		xmlResponse = xmlSlurpur.parseText(response.responseBodyContent)

		KeywordUtil.logInfo('The PNR Contents are   \n' + xmlResponse.Body.CommandRS.HCAResponse.HCAA_Data.HCAA_Payload.HCAATerm)

		String sPNR = xmlResponse.Body.CommandRS.HCAResponse.HCAA_Data.HCAA_Payload.HCAATerm.toString().substring(0, 6)

		if (sPNR.length() == 6) {

			GlobalVariable.PNR = sPNR
			KeywordUtil.logInfo('The PNR is '+ GlobalVariable.PNR)

		}

		else

		{

			KeywordUtil.markFailedAndStop('The PNR is not generated, kindly check the log')
		}

		KeywordUtil.logInfo('--------Processing Pricing Entry-----------')

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : FareEntry, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		KeywordUtil.logInfo('--------Processing Form of Payment Entry-----------')
		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : sFoP, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : SSRDocs, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : sReceivedfrom, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : AddCommit, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : IgnoreandRetreive, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		KeywordUtil.logInfo('--------Processing Ticketing Entry-----------')

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : 'tkp', ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))


		if (xmlResponse.Body.CommandRS.HCAResponse.HCAA_Data.HCAA_Payload.HCAATerm.toString().contains("CONFIRM SEGMENT")) {


			WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
				, ('Dynamiccomment') : 'er', ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))


		}

		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : 'I', ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))




		def PNR = '*'+GlobalVariable.PNR


		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : PNR, ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

		KeywordUtil.logInfo('--------Processing Etkt check Entry-----------')
		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : '*HTE', ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))


		response = WSResponseManager.getInstance().getCurrentResponse()

		xmlSlurpur = new XmlSlurper()

		xmlResponse = xmlSlurpur.parseText(response.responseBodyContent)


		//println('The Ticket Contents are   \n' + xmlResponse)

		String sTicket = xmlResponse.Body.CommandRS.HCAResponse.HCAA_Data.HCAA_Payload.HCAATerm.toString().substring(5, 22)
		sTicket = sTicket.trim().replaceAll(" ", "")
		//sTicket = sTicket.replace(" ", "")

		if (sTicket.length() == 13) {

			GlobalVariable.sOldTicket = sTicket
			KeywordUtil.logInfo('The Ticket Number no is '+ GlobalVariable.sOldTicket )

		}

		else
		{

			KeywordUtil.markFailedAndStop('The Ticket is not generated, kindly check the log')
		}





		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_Dynamiccomments', [('ExtSessionToken') : GlobalVariable.hca_ExtSessiontoken
			, ('Dynamiccomment') : 'I', ('pcc') : GlobalVariable.sPCC, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))


		WS.sendRequestAndVerify(findTestObject('Hcaservices/HCA_logout', [('SessionToken') : GlobalVariable.hca_ExtSessiontoken, ('pcc') : GlobalVariable.sPCC
			, ('customerProfileId') : GlobalVariable.sCustomerProfileId]))

	}



}





